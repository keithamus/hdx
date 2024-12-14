use hdx_atom::atom;
use hdx_lexer::{Cursor, Kind, Token};
use hdx_parser::{
	diagnostics, keyword_typedef, Build, CursorSink, Is, Parse, Parser, Peek, Result as ParserResult, ToCursors, T,
};

use crate::css::units::LengthPercentage;

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Position {
	SingleValue(PositionSingleValue),
	TwoValue(PositionHorizontal, PositionVertical),
	FourValue(PositionHorizontalKeyword, LengthPercentage, PositionVerticalKeyword, LengthPercentage),
}

impl<'a> Peek<'a> for Position {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<PositionSingleValue>()
	}
}

impl<'a> Parse<'a> for Position {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let first = p.parse::<PositionSingleValue>()?;
		// Single case
		if !p.peek::<PositionSingleValue>() {
			return Ok(Self::SingleValue(first));
		}
		let second = p.parse::<PositionSingleValue>()?;
		// Two value
		if !p.peek::<PositionSingleValue>() {
			if let Some(horizontal) = first.to_horizontal() {
				if let Some(vertical) = second.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				}
			} else if let Some(horizontal) = second.to_horizontal() {
				if let Some(vertical) = first.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				} else {
					let cursor: Cursor = second.into();
					Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
				}
			}
		}
		// Four value
		if matches!(first, PositionSingleValue::Center(_) | PositionSingleValue::LengthPercentage(_))
			|| !matches!(&second, PositionSingleValue::LengthPercentage(_))
		{
			let cursor: Cursor = second.into();
			Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
		}
		let third = p.parse::<PositionSingleValue>()?;
		if third.to_horizontal_keyword().is_none() && third.to_vertical_keyword().is_none() {
			let cursor: Cursor = third.into();
			Err(diagnostics::UnexpectedIdent(p.parse_atom_lower(cursor), cursor.into()))?
		}
		let fourth = p.parse::<LengthPercentage>()?;
		if let PositionSingleValue::LengthPercentage(second) = second {
			if let Some(horizontal) = first.to_horizontal_keyword() {
				if let Some(vertical) = third.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, second, vertical, fourth))
				} else {
					let cursor: Cursor = third.into();
					Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
				}
			} else if let Some(horizontal) = third.to_horizontal_keyword() {
				if let Some(vertical) = first.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, fourth, vertical, second))
				} else {
					let cursor: Cursor = third.into();
					Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
				}
			} else {
				let cursor: Cursor = third.into();
				Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
			}
		} else {
			let cursor: Cursor = second.into();
			Err(diagnostics::Unexpected(cursor.into(), cursor.into()))?
		}
	}
}

impl<'a> ToCursors for Position {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match *self {
			Self::SingleValue(v) => {
				s.append(v.into());
			}
			Self::TwoValue(a, b) => {
				s.append(a.into());
				s.append(b.into());
			}
			Self::FourValue(a, b, c, d) => {
				s.append(a.into());
				s.append(b.into());
				s.append(c.into());
				s.append(d.into());
			}
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionSingleValue {
	Left(T![Ident]),
	Right(T![Ident]),
	Center(T![Ident]),
	Top(T![Ident]),
	Bottom(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl PositionSingleValue {
	#[inline]
	fn to_horizontal(self) -> Option<PositionHorizontal> {
		match self {
			Self::Left(t) => Some(PositionHorizontal::Left(t)),
			Self::Right(t) => Some(PositionHorizontal::Right(t)),
			Self::Center(t) => Some(PositionHorizontal::Center(t)),
			Self::LengthPercentage(l) => Some(PositionHorizontal::LengthPercentage(l)),
			_ => None,
		}
	}

	#[inline]
	fn to_vertical(self) -> Option<PositionVertical> {
		match self {
			Self::Top(t) => Some(PositionVertical::Top(t)),
			Self::Bottom(t) => Some(PositionVertical::Bottom(t)),
			Self::Center(t) => Some(PositionVertical::Center(t)),
			Self::LengthPercentage(l) => Some(PositionVertical::LengthPercentage(l)),
			_ => None,
		}
	}

	#[inline]
	fn to_horizontal_keyword(self) -> Option<PositionHorizontalKeyword> {
		match self {
			Self::Left(t) => Some(PositionHorizontalKeyword::Left(t.into())),
			Self::Right(t) => Some(PositionHorizontalKeyword::Right(t.into())),
			_ => None,
		}
	}

	#[inline]
	fn to_vertical_keyword(self) -> Option<PositionVerticalKeyword> {
		match self {
			Self::Top(t) => Some(PositionVerticalKeyword::Top(t.into())),
			Self::Bottom(t) => Some(PositionVerticalKeyword::Bottom(t.into())),
			_ => None,
		}
	}
}

impl From<PositionSingleValue> for Token {
	fn from(value: PositionSingleValue) -> Self {
		match value {
			PositionSingleValue::Left(v) => v.into(),
			PositionSingleValue::Right(v) => v.into(),
			PositionSingleValue::Center(v) => v.into(),
			PositionSingleValue::Top(v) => v.into(),
			PositionSingleValue::Bottom(v) => v.into(),
			PositionSingleValue::LengthPercentage(v) => v.into(),
		}
	}
}

impl From<PositionSingleValue> for Cursor {
	fn from(value: PositionSingleValue) -> Self {
		match value {
			PositionSingleValue::Left(v) => v.into(),
			PositionSingleValue::Right(v) => v.into(),
			PositionSingleValue::Center(v) => v.into(),
			PositionSingleValue::Top(v) => v.into(),
			PositionSingleValue::Bottom(v) => v.into(),
			PositionSingleValue::LengthPercentage(v) => v.into(),
		}
	}
}

impl From<PositionSingleValue> for Kind {
	fn from(value: PositionSingleValue) -> Self {
		let t: Token = value.into();
		t.into()
	}
}

impl<'a> Is<'a> for PositionSingleValue {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::is(p, c)
			|| (<T![Ident]>::is(p, c)
				&& matches!(
					p.parse_atom_lower(c),
					atom!("left") | atom!("right") | atom!("top") | atom!("bottom") | atom!("center")
				))
	}
}

impl<'a> Build<'a> for PositionSingleValue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Ident]>::is(p, c) {
			let ident = <T![Ident]>::build(p, c);
			match p.parse_atom_lower(c) {
				atom!("center") => Self::Center(ident),
				atom!("left") => Self::Left(ident),
				atom!("right") => Self::Right(ident),
				atom!("top") => Self::Top(ident),
				atom!("bottom") => Self::Bottom(ident),
				_ => unreachable!(),
			}
		} else if LengthPercentage::is(p, c) {
			Self::LengthPercentage(LengthPercentage::build(p, c))
		} else {
			unreachable!()
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionHorizontal {
	Left(T![Ident]),
	Right(T![Ident]),
	Center(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl From<PositionHorizontal> for Token {
	fn from(value: PositionHorizontal) -> Self {
		match value {
			PositionHorizontal::Left(v) => v.into(),
			PositionHorizontal::Right(v) => v.into(),
			PositionHorizontal::Center(v) => v.into(),
			PositionHorizontal::LengthPercentage(v) => v.into(),
		}
	}
}

impl From<PositionHorizontal> for Cursor {
	fn from(value: PositionHorizontal) -> Self {
		match value {
			PositionHorizontal::Left(v) => v.into(),
			PositionHorizontal::Right(v) => v.into(),
			PositionHorizontal::Center(v) => v.into(),
			PositionHorizontal::LengthPercentage(v) => v.into(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionVertical {
	Top(T![Ident]),
	Bottom(T![Ident]),
	Center(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl From<PositionVertical> for Token {
	fn from(value: PositionVertical) -> Self {
		match value {
			PositionVertical::Top(v) => v.into(),
			PositionVertical::Bottom(v) => v.into(),
			PositionVertical::Center(v) => v.into(),
			PositionVertical::LengthPercentage(v) => v.into(),
		}
	}
}

impl From<PositionVertical> for Cursor {
	fn from(value: PositionVertical) -> Self {
		match value {
			PositionVertical::Top(v) => v.into(),
			PositionVertical::Bottom(v) => v.into(),
			PositionVertical::Center(v) => v.into(),
			PositionVertical::LengthPercentage(v) => v.into(),
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
		assert_size!(Position, 56);
	}

	#[test]
	fn test_writes() {
		// assert_parse!(Position, "left");
		// assert_parse!(Position, "right");
		// assert_parse!(Position, "top");
		// assert_parse!(Position, "bottom");
		// assert_parse!(Position, "center");
		assert_parse!(Position, "center center");
		// assert_parse!(Position, "center top");
		// assert_parse!(Position, "50% 50%");
		// assert_parse!(Position, "50%");
		// assert_parse!(Position, "20px 30px");
		// assert_parse!(Position, "2% bottom");
		// assert_parse!(Position, "-70% -180%");
		// assert_parse!(Position, "right 8.5%");
		// assert_parse!(Position, "right -6px bottom 12vmin");
		// assert_parse!(Position, "bottom 12vmin right -6px", "right -6px bottom 12vmin");
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
