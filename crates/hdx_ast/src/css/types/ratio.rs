use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{match_ignore_case, unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::LengthPercentage;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Ratio(pub HorizontalRatio, pub VerticalRatio);

impl<'a> Parse<'a> for Ratio {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let maybe_horizontal = if match_ignore_case!(parser.peek(), Token::Ident(atom!("top") | atom!("bottom"))) {
			None
		} else {
			HorizontalRatio::parse(parser).ok()
		};
		let vertical = VerticalRatio::parse(parser).unwrap_or_else(|_| {
			if matches!(maybe_horizontal, Some(HorizontalRatio::LengthPercentage(_))) {
				VerticalRatio::LengthPercentage(LengthPercentage::Percent(50.0.into()))
			} else {
				VerticalRatio::default()
			}
		});
		let horizontal = maybe_horizontal
			.unwrap_or_else(|| HorizontalRatio::parse(parser).unwrap_or_else(|_| HorizontalRatio::default()));
		// Horizontal cannot have a Top/Bottom with a length, if Vertical does not also (IOW no three-value syntax)
		if (matches!(horizontal, HorizontalRatio::Left(Some(_)) | HorizontalRatio::Right(Some(_)))
			&& matches!(vertical, VerticalRatio::Top(None) | VerticalRatio::Bottom(None)))
			|| (matches!(vertical, VerticalRatio::Top(Some(_)) | VerticalRatio::Bottom(Some(_)))
				&& matches!(horizontal, HorizontalRatio::Left(None) | HorizontalRatio::Right(None)))
		{
			unexpected!(parser);
		}
		Ok(Self(horizontal, vertical))
	}
}

impl<'a> WriteCss<'a> for Ratio {
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
pub enum HorizontalRatio {
	#[default]
	Center,
	Left(Option<LengthPercentage>),
	Right(Option<LengthPercentage>),
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for HorizontalRatio {
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

impl<'a> WriteCss<'a> for HorizontalRatio {
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
pub enum VerticalRatio {
	#[default]
	Center,
	Top(Option<LengthPercentage>),
	Bottom(Option<LengthPercentage>),
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for VerticalRatio {
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

impl<'a> WriteCss<'a> for VerticalRatio {
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
		assert_size!(Ratio, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Ratio, "left", "left center");
		assert_parse!(Ratio, "right", "right center");
		assert_parse!(Ratio, "top", "center top");
		assert_parse!(Ratio, "bottom", "center bottom");
		assert_parse!(Ratio, "center", "center center");
		assert_parse!(Ratio, "center center");
		assert_parse!(Ratio, "center top");
		assert_parse!(Ratio, "left top");
		assert_parse!(Ratio, "top left", "left top");
		assert_parse!(Ratio, "50% 50%");
		assert_parse!(Ratio, "50%", "50% 50%");
		assert_parse!(Ratio, "20px 30px");
		assert_parse!(Ratio, "2% bottom");
		assert_parse!(Ratio, "-70% -180%");
		assert_parse!(Ratio, "right 8.5%", "right 8.5% center");
		assert_parse!(Ratio, "right -6px bottom 12vmin");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Ratio, "left left");
		assert_parse_error!(Ratio, "bottom top");
		assert_parse_error!(Ratio, "10px 15px 20px 15px");
		// 3 value syntax is not allowed
		assert_parse_error!(Ratio, "right -6px bottom");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(Ratio, "center center", {
			"node": [
				{"type": "center"},
				{"type": "center"},
			],
			"start": 0,
			"end": 13
		});
		assert_json!(Ratio, "left bottom", {
			"node": [
				{"type": "left", "value": null},
				{"type": "bottom", "value": null},
			],
			"start": 0,
			"end": 11
		});
	}
}
