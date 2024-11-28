use hdx_parser::{keyword_typedef, CursorStream, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub(crate) use crate::css::units::*;

mod func {
	use hdx_parser::custom_function;

	custom_function!(SnapBlock, atom!("snap-block"));
	custom_function!(SnapInline, atom!("snap-inline"));
}

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlock {
	pub function: func::SnapBlock,
	pub length: LengthPercentage,
	pub comma: Option<T![,]>,
	pub keyword: Option<SnapBlockKeyword>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for SnapBlock {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::SnapBlock>()
	}
}

impl<'a> Parse<'a> for SnapBlock {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<func::SnapBlock>()?;
		let length = p.parse::<LengthPercentage>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let keyword = p.parse_if_peek::<SnapBlockKeyword>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { function, length, comma, keyword, close })
	}
}

impl<'a> ToCursors<'a> for SnapBlock {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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

keyword_typedef!(SnapBlockKeyword { Start: atom!("start"), End: atom!("end"), Near: atom!("near") });

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline
// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapInline {
	pub function: func::SnapInline,
	pub length: LengthPercentage,
	pub comma: Option<T![,]>,
	pub keyword: Option<SnapInlineKeyword>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for SnapInline {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::SnapInline>()
	}
}

impl<'a> Parse<'a> for SnapInline {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<func::SnapInline>()?;
		let length = p.parse::<LengthPercentage>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let keyword = p.parse_if_peek::<SnapInlineKeyword>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { function, length, comma, keyword, close })
	}
}

impl<'a> ToCursors<'a> for SnapInline {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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
keyword_typedef!(SnapInlineKeyword { Left: atom!("left"), Right: atom!("right"), Near: atom!("near") });
