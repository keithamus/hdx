use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::Value;

use crate::css::values::units::Angle;

// https://drafts.csswg.org/css-fonts/#font-style-prop
#[derive(Value, Default, Debug, PartialEq, Hash)]
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
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("normal") => {
					parser.advance();
					Ok(Self::Normal)
				}
				atom!("italic") => {
					parser.advance();
					Ok(Self::Italic)
				}
				atom!("oblique") => {
					parser.advance();
					match parser.cur() {
						Token::Dimension(val, unit, _) => {
							if !matches!(unit.to_ascii_lowercase(), atom!("deg")) {
								Err(diagnostics::UnexpectedDimension(unit, parser.span()))?
							}
							if !(-90.0..=90.0).contains(&val) {
								Err(diagnostics::NumberOutOfBounds(val, "-90..=90".into(), parser.span()))?
							}
							parser.advance();
							Ok(Self::ObliqueAngle(Angle::Deg(val.into())))
						}
						_ => Ok(Self::Oblique),
					}
				}
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
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
				atom!("oblique").write_css(sink)?;
				sink.write_char(' ')?;
				deg.write_css(sink)
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
