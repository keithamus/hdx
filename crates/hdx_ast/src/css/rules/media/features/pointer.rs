#[cfg(feature = "serde")]
use serde::Serialize;

use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, MediaFeature, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Default, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PointerMediaFeature {
	#[default]
	Any,
	None,
	Coarse,
	Fine,
}

impl<'a> Parse<'a> for PointerMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_media_feature(atom!("pointer"), parser)
	}
}

impl<'a> MediaFeature<'a> for PointerMediaFeature {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Ok(Self::None)
				}
				atom!("coarse") => {
					parser.advance();
					Ok(Self::Coarse)
				}
				atom!("fine") => {
					parser.advance();
					Ok(Self::Fine)
				}
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for PointerMediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('(')?;
		atom!("pointer").write_css(sink)?;
		match self {
			Self::None => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("none").write_css(sink)?;
			}
			Self::Coarse => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("coarse").write_css(sink)?;
			}
			Self::Fine => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("fine").write_css(sink)?;
			}
			Self::Any => {}
		}
		sink.write_char(')')
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PointerMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PointerMediaFeature, "(pointer)");
		assert_parse!(PointerMediaFeature, "(pointer: none)");
		assert_parse!(PointerMediaFeature, "(pointer: coarse)");
		assert_parse!(PointerMediaFeature, "(pointer: fine)");
	}

	#[test]
	fn test_minify() {
		assert_minify!(PointerMediaFeature, "(pointer: none)", "(pointer:none)");
		assert_minify!(PointerMediaFeature, "(pointer: coarse)", "(pointer:coarse)");
		assert_minify!(PointerMediaFeature, "(pointer: fine)", "(pointer:fine)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PointerMediaFeature, "(pointer:)");
		assert_parse_error!(PointerMediaFeature, "(pointer: pointer)");
	}
}
