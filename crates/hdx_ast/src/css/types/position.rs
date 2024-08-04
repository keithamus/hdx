use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::LengthPercentage;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Position(pub HorizontalPosition, pub VerticalPosition);

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Top, atom!("top"));
	custom_keyword!(Right, atom!("right"));
	custom_keyword!(Bottom, atom!("bottom"));
	custom_keyword!(Left, atom!("left"));
	custom_keyword!(Center, atom!("center"));
}

impl<'a> Parse<'a> for Position {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.peek::<Token![Any]>().unwrap();
		let maybe_horizontal = parser.parse_if_peek::<HorizontalPosition>().ok().flatten();
		let vertical = parser.parse::<VerticalPosition>().unwrap_or_else(|_| {
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
			Err(diagnostics::Unexpected(token, token.span()))?
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

impl<'a> Peek<'a> for HorizontalPosition {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser
			.peek::<kw::Left>()
			.or_else(|| parser.peek::<kw::Right>())
			.or_else(|| parser.peek::<kw::Top>())
			.or_else(|| parser.peek::<LengthPercentage>())
	}
}

impl<'a> Parse<'a> for HorizontalPosition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Center>() {
			parser.hop(token);
			Ok(Self::Center)
		} else if let Some(token) = parser.peek::<kw::Left>() {
			parser.hop(token);
			let len = parser.parse_if_peek::<LengthPercentage>().ok().flatten();
			Ok(Self::Left(len))
		} else if let Some(token) = parser.peek::<kw::Right>() {
			parser.hop(token);
			let len = parser.parse_if_peek::<LengthPercentage>().ok().flatten();
			Ok(Self::Right(len))
		} else {
			parser.parse::<LengthPercentage>().map(Self::LengthPercentage)
		}
	}
}

impl<'a> WriteCss<'a> for HorizontalPosition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => write_css!(sink, kw::Center::atom()),
			Self::Left(pos) => {
				write_css!(sink, kw::Left::atom());
				if let Some(pos) = pos {
					write_css!(sink, ' ', pos);
				}
			}
			Self::Right(pos) => {
				write_css!(sink, kw::Right::atom());
				if let Some(pos) = pos {
					write_css!(sink, ' ', pos);
				}
			}
			Self::LengthPercentage(l) => write_css!(sink, l),
		}
		Ok(())
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

impl<'a> Peek<'a> for VerticalPosition {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser
			.peek::<kw::Left>()
			.or_else(|| parser.peek::<kw::Right>())
			.or_else(|| parser.peek::<kw::Top>())
			.or_else(|| parser.peek::<LengthPercentage>())
	}
}

impl<'a> Parse<'a> for VerticalPosition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Center>() {
			parser.hop(token);
			Ok(Self::Center)
		} else if let Some(token) = parser.peek::<kw::Top>() {
			parser.hop(token);
			let len = parser.parse_if_peek::<LengthPercentage>().ok().flatten();
			Ok(Self::Top(len))
		} else if let Some(token) = parser.peek::<kw::Bottom>() {
			parser.hop(token);
			let len = parser.parse_if_peek::<LengthPercentage>().ok().flatten();
			Ok(Self::Bottom(len))
		} else {
			parser.parse::<LengthPercentage>().map(Self::LengthPercentage)
		}
	}
}

impl<'a> WriteCss<'a> for VerticalPosition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => write_css!(sink, kw::Center::atom()),
			Self::Top(pos) => {
				write_css!(sink, kw::Top::atom());
				if let Some(pos) = pos {
					write_css!(sink, ' ', pos);
				}
			}
			Self::Bottom(pos) => {
				write_css!(sink, kw::Bottom::atom());
				if let Some(pos) = pos {
					write_css!(sink, ' ', pos);
				}
			}
			Self::LengthPercentage(l) => write_css!(sink, l),
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
