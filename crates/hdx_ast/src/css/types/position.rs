use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{match_ignore_case, unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::LengthPercentage;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Position(pub HorizontalPosition, pub VerticalPosition);

impl<'a> Parse<'a> for Position {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let maybe_horizontal = if match_ignore_case!(parser.peek(), Token::Ident(atom!("top") | atom!("bottom"))) {
			None
		} else {
			HorizontalPosition::parse(parser).ok()
		};
		let vertical = VerticalPosition::parse(parser).unwrap_or_else(|_| {
			if matches!(maybe_horizontal, Some(HorizontalPosition::LengthPercentage(_))) {
				VerticalPosition::LengthPercentage(LengthPercentage::Percent(50.0.into()))
			} else {
				VerticalPosition::default()
			}
		});
		let horizontal = maybe_horizontal
			.unwrap_or_else(|| HorizontalPosition::parse(parser).unwrap_or_else(|_| HorizontalPosition::default()));
		// Horizontal cannot have a Top/Bottom with a length, if Vertical does not also (IOW no three-value syntax)
		if (matches!(horizontal, HorizontalPosition::Left(Some(_)) | HorizontalPosition::Right(Some(_)))
			&& matches!(vertical, VerticalPosition::Top(None) | VerticalPosition::Bottom(None)))
			|| (matches!(vertical, VerticalPosition::Top(Some(_)) | VerticalPosition::Bottom(Some(_)))
				&& matches!(horizontal, HorizontalPosition::Left(None) | HorizontalPosition::Right(None)))
		{
			unexpected!(parser);
		}
		Ok(Self(horizontal, vertical))
	}
}

impl<'a> WriteCss<'a> for Position {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)?;
		sink.write_char(' ')?;
		self.1.write_css(sink)
	}
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum HorizontalPosition {
	#[default]
	Center,
	Left(Option<LengthPercentage>),
	Right(Option<LengthPercentage>),
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for HorizontalPosition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("center") => {
					parser.advance();
					Self::Center
				}
				atom!("left") => {
					parser.advance();
					let len = LengthPercentage::from_token(parser.peek());
					if len.is_some() {
						parser.advance();
					}
					Self::Left(len)
				}
				atom!("right") => {
					parser.advance();
					let len = LengthPercentage::from_token(parser.peek());
					if len.is_some() {
						parser.advance();
					}
					Self::Right(len)
				}
				_ => unexpected_ident!(parser, atom),
			},
			_ => Self::LengthPercentage(LengthPercentage::parse(parser)?),
		})
	}
}

impl<'a> WriteCss<'a> for HorizontalPosition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => atom!("center").write_css(sink),
			Self::Left(pos) => {
				atom!("left").write_css(sink)?;
				if let Some(pos) = pos {
					sink.write_char(' ')?;
					pos.write_css(sink)?;
				}
				Ok(())
			}
			Self::Right(pos) => {
				atom!("right").write_css(sink)?;
				if let Some(pos) = pos {
					sink.write_char(' ')?;
					pos.write_css(sink)?;
				}
				Ok(())
			}
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum VerticalPosition {
	#[default]
	Center,
	Top(Option<LengthPercentage>),
	Bottom(Option<LengthPercentage>),
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for VerticalPosition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("center") => {
					parser.advance();
					Self::Center
				}
				atom!("top") => {
					parser.advance();
					let len = LengthPercentage::from_token(parser.peek());
					if len.is_some() {
						parser.advance();
					}
					Self::Top(len)
				}
				atom!("bottom") => {
					parser.advance();
					let len = LengthPercentage::from_token(parser.peek());
					if len.is_some() {
						parser.advance();
					}
					Self::Bottom(len)
				}
				_ => unexpected_ident!(parser, atom),
			},
			_ => Self::LengthPercentage(LengthPercentage::parse(parser)?),
		})
	}
}

impl<'a> WriteCss<'a> for VerticalPosition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => atom!("center").write_css(sink),
			Self::Top(pos) => {
				atom!("top").write_css(sink)?;
				if let Some(pos) = pos {
					sink.write_char(' ')?;
					pos.write_css(sink)?;
				}
				Ok(())
			}
			Self::Bottom(pos) => {
				atom!("bottom").write_css(sink)?;
				if let Some(pos) = pos {
					sink.write_char(' ')?;
					pos.write_css(sink)?;
				}
				Ok(())
			}
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Position, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Position, "left", "left center");
		assert_parse!(Position, "right", "right center");
		assert_parse!(Position, "top", "center top");
		assert_parse!(Position, "bottom", "center bottom");
		assert_parse!(Position, "center", "center center");
		assert_parse!(Position, "center center");
		assert_parse!(Position, "center top");
		assert_parse!(Position, "left top");
		assert_parse!(Position, "top left", "left top");
		assert_parse!(Position, "50% 50%");
		assert_parse!(Position, "50%", "50% 50%");
		assert_parse!(Position, "20px 30px");
		assert_parse!(Position, "2% bottom");
		assert_parse!(Position, "-70% -180%");
		assert_parse!(Position, "right 8.5%", "right 8.5% center");
		assert_parse!(Position, "right -6px bottom 12vmin");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Position, "left left");
		assert_parse_error!(Position, "bottom top");
		assert_parse_error!(Position, "10px 15px 20px 15px");
		// 3 value syntax is not allowed
		assert_parse_error!(Position, "right -6px bottom");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(Position, "center center", {
			"node": [
				{"type": "center"},
				{"type": "center"},
			],
			"start": 0,
			"end": 13
		});
		assert_json!(Position, "left bottom", {
			"node": [
				{"type": "left", "value": null},
				{"type": "bottom", "value": null},
			],
			"start": 0,
			"end": 11
		});
	}
}
