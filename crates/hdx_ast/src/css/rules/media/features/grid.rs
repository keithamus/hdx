use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, DiscreteMediaFeature, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Default, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum GridMediaFeature {
	#[default]
	Zero,
	One,
}

impl<'a> Parse<'a> for GridMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_descrete_media_feature(atom!("grid"), parser)
	}
}

impl<'a> DiscreteMediaFeature<'a> for GridMediaFeature {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.next() {
			Token::Number(val, ty) => {
				if *val == 1.0 && ty.is_int() {
					Ok(Self::One)
				} else if *val == 0.0 && ty.is_int() {
					Ok(Self::Zero)
				} else {
					unexpected!(parser)
				}
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for GridMediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		atom!("grid").write_css(sink)?;
		if matches!(self, Self::One) {
			sink.write_char(':')?;
			sink.write_whitespace()?;
			sink.write_char('1')?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(GridMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridMediaFeature, "grid: 1");
		assert_parse!(GridMediaFeature, "grid");
	}

	#[test]
	fn test_minify() {
		assert_minify!(GridMediaFeature, "grid: 1", "grid:1");
		assert_minify!(GridMediaFeature, "grid: 0", "grid");
	}
}
