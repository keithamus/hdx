use hdx_atom::atom;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::{css::units::LengthPercentage, macros::keyword_typedef};

// https://drafts.csswg.org/css-values-4/#position
// <position> = [
//   [ left | center | right | top | bottom | <length-percentage> ]
// |
//   [ left | center | right ] && [ top | center | bottom ]
// |
//   [ left | center | right | <length-percentage> ]
//   [ top | center | bottom | <length-percentage> ]
// |
//   [ [ left | right ] <length-percentage> ] &&
//   [ [ top | bottom ] <length-percentage> ]
// ]
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Position {
	SingleValue(PositionSingleValue),
	TwoValue(PositionHorizontal, PositionVertical),
	FourValue(PositionHorizontalKeyword, LengthPercentage, PositionVerticalKeyword, LengthPercentage),
}

impl<'a> Parse<'a> for Position {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let first = p.parse::<PositionSingleValue>()?;
		let peek_second = p.peek::<PositionSingleValue>();
		// Single case
		if peek_second.is_none() {
			return Ok(Self::SingleValue(first));
		}
		let second = dbg!(p.parse::<PositionSingleValue>())?;
		let peek_third = p.peek::<T![Ident]>();
		// Two value
		if peek_third.is_none() {
			if let Some(horizontal) = first.to_horizontal() {
				if let Some(vertical) = second.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				}
			} else if let Some(horizontal) = second.to_horizontal() {
				if let Some(vertical) = first.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				} else {
					Err(diagnostics::Unexpected(peek_second.unwrap(), peek_second.unwrap().span()))?
				}
			}
		}
		// Four value
		if matches!(first, PositionSingleValue::Center | PositionSingleValue::LengthPercentage(_))
			|| !matches!(second, PositionSingleValue::LengthPercentage(_))
		{
			Err(diagnostics::Unexpected(peek_second.unwrap(), peek_second.unwrap().span()))?
		}
		if peek_third.is_none() {
			let token = p.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
		let third = p.parse::<PositionSingleValue>()?;
		if third.to_horizontal_keyword().is_none() && third.to_vertical_keyword().is_none() {
			Err(diagnostics::UnexpectedIdent(p.parse_atom_lower(peek_third.unwrap()), peek_third.unwrap().span()))?
		}
		let fourth = p.parse::<LengthPercentage>()?;
		if let PositionSingleValue::LengthPercentage(second) = second {
			if let Some(horizontal) = first.to_horizontal_keyword() {
				if let Some(vertical) = third.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, second, vertical, fourth))
				} else {
					Err(diagnostics::Unexpected(peek_third.unwrap(), peek_third.unwrap().span()))?
				}
			} else if let Some(horizontal) = third.to_horizontal_keyword() {
				if let Some(vertical) = first.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, fourth, vertical, second))
				} else {
					Err(diagnostics::Unexpected(peek_third.unwrap(), peek_third.unwrap().span()))?
				}
			} else {
				Err(diagnostics::Unexpected(peek_third.unwrap(), peek_third.unwrap().span()))?
			}
		} else {
			Err(diagnostics::Unexpected(peek_second.unwrap(), peek_second.unwrap().span()))?
		}
	}
}

impl<'a> WriteCss<'a> for Position {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::SingleValue(a) => write_css!(sink, a),
			Self::TwoValue(a, b) => write_css!(sink, a, ' ', b),
			Self::FourValue(a, b, c, d) => write_css!(sink, a, ' ', b, ' ', c, ' ', d),
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionSingleValue {
	Left,
	Right,
	Center,
	Top,
	Bottom,
	LengthPercentage(LengthPercentage),
}

impl PositionSingleValue {
	#[inline]
	pub fn to_horizontal(&self) -> Option<PositionHorizontal> {
		match self {
			Self::Left => Some(PositionHorizontal::Left),
			Self::Right => Some(PositionHorizontal::Right),
			Self::Center => Some(PositionHorizontal::Center),
			Self::LengthPercentage(l) => Some(PositionHorizontal::LengthPercentage(*l)),
			_ => None,
		}
	}

	#[inline]
	pub fn to_vertical(&self) -> Option<PositionVertical> {
		match self {
			Self::Top => Some(PositionVertical::Top),
			Self::Bottom => Some(PositionVertical::Bottom),
			Self::Center => Some(PositionVertical::Center),
			Self::LengthPercentage(l) => Some(PositionVertical::LengthPercentage(*l)),
			_ => None,
		}
	}

	#[inline]
	pub fn to_horizontal_keyword(&self) -> Option<PositionHorizontalKeyword> {
		match self {
			Self::Left => Some(PositionHorizontalKeyword::Left),
			Self::Right => Some(PositionHorizontalKeyword::Right),
			_ => None,
		}
	}

	#[inline]
	pub fn to_vertical_keyword(&self) -> Option<PositionVerticalKeyword> {
		match self {
			Self::Top => Some(PositionVerticalKeyword::Top),
			Self::Bottom => Some(PositionVerticalKeyword::Bottom),
			_ => None,
		}
	}
}

impl<'a> Peek<'a> for PositionSingleValue {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Ident]>()
			.filter(|token| {
				matches!(
					p.parse_atom_lower(*token),
					atom!("left") | atom!("right") | atom!("top") | atom!("bottom") | atom!("center")
				)
			})
			.or_else(|| p.peek::<LengthPercentage>())
	}
}

impl<'a> Parse<'a> for PositionSingleValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(length) = p.parse_if_peek::<LengthPercentage>()? {
			return Ok(Self::LengthPercentage(length));
		}
		let token = *p.parse::<T![Ident]>()?;
		match p.parse_atom_lower(token) {
			atom!("center") => Ok(Self::Center),
			atom!("left") => Ok(Self::Left),
			atom!("right") => Ok(Self::Right),
			atom!("top") => Ok(Self::Top),
			atom!("bottom") => Ok(Self::Bottom),
			atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
		}
	}
}
impl<'a> WriteCss<'a> for PositionSingleValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => atom!("center").write_css(sink),
			Self::Left => atom!("left").write_css(sink),
			Self::Right => atom!("right").write_css(sink),
			Self::Top => atom!("top").write_css(sink),
			Self::Bottom => atom!("bottom").write_css(sink),
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionHorizontal {
	Left,
	Right,
	Center,
	LengthPercentage(LengthPercentage),
}

impl<'a> WriteCss<'a> for PositionHorizontal {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => atom!("center").write_css(sink),
			Self::Left => atom!("left").write_css(sink),
			Self::Right => atom!("right").write_css(sink),
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionVertical {
	Top,
	Bottom,
	Center,
	LengthPercentage(LengthPercentage),
}

impl<'a> WriteCss<'a> for PositionVertical {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Center => atom!("center").write_css(sink),
			Self::Top => atom!("top").write_css(sink),
			Self::Bottom => atom!("bottom").write_css(sink),
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

keyword_typedef!(PositionHorizontalKeyword { Left: atom!("left"), Right: atom!("right") });

keyword_typedef!(PositionVerticalKeyword { Top: atom!("top"), Bottom: atom!("bottom") });

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Position, 20);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Position, "left");
		assert_parse!(Position, "right");
		assert_parse!(Position, "top");
		assert_parse!(Position, "bottom");
		assert_parse!(Position, "center");
		assert_parse!(Position, "center center");
		assert_parse!(Position, "center top");
		assert_parse!(Position, "50% 50%");
		assert_parse!(Position, "50%");
		assert_parse!(Position, "20px 30px");
		assert_parse!(Position, "2% bottom");
		assert_parse!(Position, "-70% -180%");
		assert_parse!(Position, "right 8.5%");
		assert_parse!(Position, "right -6px bottom 12vmin");
		assert_parse!(Position, "bottom 12vmin right -6px", "right -6px bottom 12vmin");
	}

	// #[test]
	// fn test_errors() {
	// 	assert_parse_error!(Position, "left left");
	// 	assert_parse_error!(Position, "bottom top");
	// 	assert_parse_error!(Position, "10px 15px 20px 15px");
	// 	// 3 value syntax is not allowed
	// 	assert_parse_error!(Position, "right -6px bottom");
	// }
	//
	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Position, "center center", {
	// 		"node": [
	// 			{"type": "center"},
	// 			{"type": "center"},
	// 		],
	// 		"start": 0,
	// 		"end": 13
	// 	});
	// 	assert_json!(Position, "left bottom", {
	// 		"node": [
	// 			{"type": "left", "value": null},
	// 			{"type": "bottom", "value": null},
	// 		],
	// 		"start": 0,
	// 		"end": 11
	// 	});
	// }
}
