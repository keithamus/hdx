use css_lexer::Cursor;
use css_parse::{diagnostics, keyword_set, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub(crate) type AnchorName = T![DashedIdent];

// https://drafts.csswg.org/css-anchor-position-1/#typedef-try-size
// <try-size> = most-width | most-height | most-block-size | most-inline-size
keyword_set!(TrySize {
	MostWidth: "most-width",
	MostHeight: "most-height",
	MostBlockSize: "most-block-size",
	MostInlineSize: "most-inline-size",
});

// https://drafts.csswg.org/css-anchor-position-1/#typedef-position-area
// <position-area> = [
//   [ left | center | right | span-left | span-right
//   | x-start | x-end | span-x-start | span-x-end
//   | x-self-start | x-self-end | span-x-self-start | span-x-self-end
//   | span-all ]
//   ||
//   [ top | center | bottom | span-top | span-bottom
//   | y-start | y-end | span-y-start | span-y-end
//   | y-self-start | y-self-end | span-y-self-start | span-y-self-end
//   | span-all ]
// |
//   [ block-start | center | block-end | span-block-start | span-block-end | span-all ]
//   ||
//   [ inline-start | center | inline-end | span-inline-start | span-inline-end
//   | span-all ]
// |
//   [ self-block-start | center | self-block-end | span-self-block-start
//   | span-self-block-end | span-all ]
//   ||
//   [ self-inline-start | center | self-inline-end | span-self-inline-start
//   | span-self-inline-end | span-all ]
// |
//   [ start | center | end | span-start | span-end | span-all ]{1,2}
// |
//   [ self-start | center | self-end | span-self-start | span-self-end | span-all ]{1,2}
// ]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PositionArea {
	Physical(Option<PositionAreaPhsyicalHorizontal>, Option<PositionAreaPhsyicalVertical>),
	Logical(Option<PositionAreaBlock>, Option<PositionAreaInline>),
	SelfLogical(Option<PositionAreaSelfBlock>, Option<PositionAreaSelfInline>),
	Position(PositionAreaPosition, Option<PositionAreaPosition>),
	SelfPosition(PositionAreaSelfPosition, Option<PositionAreaSelfPosition>),
}

impl<'a> Peek<'a> for PositionArea {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		PositionAreaPhsyicalVertical::peek(p, c)
			|| PositionAreaPhsyicalHorizontal::peek(p, c)
			|| PositionAreaBlock::peek(p, c)
			|| PositionAreaInline::peek(p, c)
			|| PositionAreaSelfBlock::peek(p, c)
			|| PositionAreaSelfInline::peek(p, c)
			|| PositionAreaPosition::peek(p, c)
			|| PositionAreaSelfPosition::peek(p, c)
	}
}

impl<'a> Parse<'a> for PositionArea {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(first) = p.parse_if_peek::<PositionAreaPosition>()? {
			Ok(Self::Position(first, p.parse_if_peek::<PositionAreaPosition>()?))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaSelfPosition>()? {
			Ok(Self::SelfPosition(first, p.parse_if_peek::<PositionAreaSelfPosition>()?))
		} else if let Some(block) = p.parse_if_peek::<PositionAreaBlock>()? {
			Ok(Self::Logical(Some(block), p.parse_if_peek::<PositionAreaInline>()?))
		} else if let Some(inline) = p.parse_if_peek::<PositionAreaInline>()? {
			Ok(Self::Logical(p.parse_if_peek::<PositionAreaBlock>()?, Some(inline)))
		} else if let Some(block) = p.parse_if_peek::<PositionAreaSelfBlock>()? {
			Ok(Self::SelfLogical(Some(block), p.parse_if_peek::<PositionAreaSelfInline>()?))
		} else if let Some(inline) = p.parse_if_peek::<PositionAreaSelfInline>()? {
			Ok(Self::SelfLogical(p.parse_if_peek::<PositionAreaSelfBlock>()?, Some(inline)))
		} else if let Some(horizontal) = p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()? {
			Ok(Self::Physical(Some(horizontal), p.parse_if_peek::<PositionAreaPhsyicalVertical>()?))
		} else if let Some(vertical) = p.parse_if_peek::<PositionAreaPhsyicalVertical>()? {
			Ok(Self::Physical(p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()?, Some(vertical)))
		} else {
			let c: Cursor = p.parse::<T![Any]>()?.into();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> ToCursors for PositionArea {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Physical(horizontal, vertical) => {
				if let Some(horizontal) = horizontal {
					ToCursors::to_cursors(horizontal, s);
				}
				if let Some(vertical) = vertical {
					ToCursors::to_cursors(vertical, s);
				}
			}
			Self::Logical(block, inline) => {
				if let Some(block) = block {
					ToCursors::to_cursors(block, s);
				}
				if let Some(inline) = inline {
					ToCursors::to_cursors(inline, s);
				}
			}
			Self::SelfLogical(block, inline) => {
				if let Some(block) = block {
					ToCursors::to_cursors(block, s);
				}
				if let Some(inline) = inline {
					ToCursors::to_cursors(inline, s);
				}
			}
			Self::Position(first, second) => {
				ToCursors::to_cursors(first, s);
				if let Some(second) = second {
					ToCursors::to_cursors(second, s);
				}
			}
			Self::SelfPosition(first, second) => {
				ToCursors::to_cursors(first, s);
				if let Some(second) = second {
					ToCursors::to_cursors(second, s);
				}
			}
		}
	}
}

keyword_set!(PositionAreaPhsyicalHorizontal {
	Left: "left",
	Center: "center",
	Right: "right",
	SpanLeft: "span-left",
	SpanRight: "span-right",
	XStart: "x-start",
	XEnd: "x-end",
	SpanXStart: "span-x-start",
	SpanXEnd: "span-x-end",
	XSelfStart: "x-self-start",
	XSelfEnd: "x-self-end",
	SpanXSelfStart: "span-x-self-start",
	SpanXSelfEnd: "span-x-self-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaPhsyicalVertical {
	Top: "top",
	Center: "center",
	Bottom: "bottom",
	SpanTop: "span-top",
	SpanBottom: "span-bottom",
	YStart: "y-start",
	YEnd: "y-end",
	SpanYStart: "span-y-start",
	SpanYEnd: "span-y-end",
	YSelfStart: "y-self-start",
	YSelfEnd: "y-self-end",
	SpanYSelfStart: "span-y-self-start",
	SpanYSelfEnd: "span-y-self-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaBlock {
	BlockStart: "block-start",
	Center: "center",
	BlockEnd: "block-end",
	SpanBlockStart: "span-block-start",
	SpanBlockEnd: "span-block-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaInline {
	InlineStart: "inline-start",
	Center: "center",
	InlineEnd: "inline-end",
	SpanInlineStart: "span-inline-start",
	SpanInlineEnd: "span-inline-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaSelfBlock {
	SelfBlockStart: "self-block-start",
	Center: "center",
	SelfBlockEnd: "self-block-end",
	SpanSelfBlockStart: "span-self-block-start",
	SpanSelfBlockEnd: "span-self-block-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaSelfInline {
	SelfInlineStart: "self-inline-start",
	Center: "center",
	SelfInlineEnd: "self-inline-end",
	SpanSelfInlineStart: "span-self-inline-start",
	SpanSelfInlineEnd: "span-self-inline-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaPosition {
	Start: "start",
	Center: "center",
	End: "end",
	SpanStart: "span-start",
	SpanEnd: "span-end",
	SpanAll: "span-all",
});

keyword_set!(PositionAreaSelfPosition {
	SelfStart: "self-start",
	Center: "center",
	SelfEnd: "self-end",
	SpanSelfStart: "span-self-start",
	SpanSelfEnd: "span-self-end",
	SpanAll: "span-all",
});
