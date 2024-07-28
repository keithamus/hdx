use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, expect_ignore_case, Parse, Parser, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::Angle;

// https://drafts.csswg.org/css-fonts/#font-style-prop
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontStyle {
	#[default]
	Normal,
	Italic,
	Oblique,
	ObliqueAngle(Angle),
}

impl<'a> Parse<'a> for FontStyle {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Ident(_):
			atom!("normal") => Ok(Self::Normal),
			atom!("italic") => Ok(Self::Italic),
			atom!("oblique") => match parser.peek().clone() {
				Token::Dimension(val, unit, _) => {
					parser.next();
					if !matches!(unit.to_ascii_lowercase(), atom!("deg")) {
						Err(diagnostics::UnexpectedDimension(unit, parser.span()))?
					}
					if !(-90.0..=90.0).contains(&val) {
						Err(diagnostics::NumberOutOfBounds(val, "-90..=90".into(), parser.span()))?
					}
					Ok(Self::ObliqueAngle(Angle::Deg(val.into())))
				}
				_ => Ok(Self::Oblique),
			}
		}
	}
}

impl<'a> WriteCss<'a> for FontStyle {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Normal => atom!("normal").write_css(sink),
			Self::Italic => atom!("italic").write_css(sink),
			Self::Oblique => atom!("oblique").write_css(sink),
			Self::ObliqueAngle(deg) => {
				write_css!(sink, atom!("oblique"), ' ', deg);
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontStyle, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontStyle, "normal");
		assert_parse!(FontStyle, "italic");
		assert_parse!(FontStyle, "oblique");
		assert_parse!(FontStyle, "oblique 20deg");
	}
}
