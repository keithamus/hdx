use css_lexer::Cursor;
use css_parse::{diagnostics, keyword_set, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub(crate) use crate::units::*;

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlock {
	pub function: T![Function],
	pub length: LengthPercentage,
	pub comma: Option<T![,]>,
	pub keyword: Option<SnapBlockKeyword>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for SnapBlock {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "snap-block")
	}
}

impl<'a> Parse<'a> for SnapBlock {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if !p.eq_ignore_ascii_case(c, "snap-block") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let length = p.parse::<LengthPercentage>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let keyword = p.parse_if_peek::<SnapBlockKeyword>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { function, length, comma, keyword, close })
	}
}

impl<'a> ToCursors for SnapBlock {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.function.into());
		s.append(self.length.into());
		if let Some(comma) = self.comma {
			s.append(comma.into());
		}
		if let Some(keyword) = self.keyword {
			s.append(keyword.into());
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

keyword_set!(SnapBlockKeyword { Start: "start", End: "end", Near: "near" });

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline
// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapInline {
	pub function: T![Function],
	pub length: LengthPercentage,
	pub comma: Option<T![,]>,
	pub keyword: Option<SnapInlineKeyword>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for SnapInline {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "snap-inline")
	}
}

impl<'a> Parse<'a> for SnapInline {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if !p.eq_ignore_ascii_case(c.into(), "snap-inline") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let length = p.parse::<LengthPercentage>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let keyword = p.parse_if_peek::<SnapInlineKeyword>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { function, length, comma, keyword, close })
	}
}

impl<'a> ToCursors for SnapInline {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.function.into());
		s.append(self.length.into());
		if let Some(comma) = self.comma {
			s.append(comma.into());
		}
		if let Some(keyword) = self.keyword {
			s.append(keyword.into());
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}
keyword_set!(SnapInlineKeyword { Left: "left", Right: "right", Near: "near" });
