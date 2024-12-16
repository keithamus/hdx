use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, keyword_typedef, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub(crate) type AnchorName = T![DashedIdent];

// https://drafts.csswg.org/css-anchor-position-1/#typedef-try-size
// <try-size> = most-width | most-height | most-block-size | most-inline-size
keyword_typedef!(TrySize {
	MostWidth: atom!("most-width"),
	MostHeight: atom!("most-height"),
	MostBlockSize: atom!("most-block-size"),
	MostInlineSize: atom!("most-inline-size"),
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
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<PositionAreaPhsyicalVertical>()
			|| p.peek::<PositionAreaPhsyicalHorizontal>()
			|| p.peek::<PositionAreaBlock>()
			|| p.peek::<PositionAreaInline>()
			|| p.peek::<PositionAreaSelfBlock>()
			|| p.peek::<PositionAreaSelfInline>()
			|| p.peek::<PositionAreaPosition>()
			|| p.peek::<PositionAreaSelfPosition>()
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

keyword_typedef!(PositionAreaPhsyicalHorizontal {
	Left: atom!("left"),
	Center: atom!("center"),
	Right: atom!("right"),
	SpanLeft: atom!("span-left"),
	SpanRight: atom!("span-right"),
	XStart: atom!("x-start"),
	XEnd: atom!("x-end"),
	SpanXStart: atom!("span-x-start"),
	SpanXEnd: atom!("span-x-end"),
	XSelfStart: atom!("x-self-start"),
	XSelfEnd: atom!("x-self-end"),
	SpanXSelfStart: atom!("span-x-self-start"),
	SpanXSelfEnd: atom!("span-x-self-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaPhsyicalVertical {
	Top: atom!("top"),
	Center: atom!("center"),
	Bottom: atom!("bottom"),
	SpanTop: atom!("span-top"),
	SpanBottom: atom!("span-bottom"),
	YStart: atom!("y-start"),
	YEnd: atom!("y-end"),
	SpanYStart: atom!("span-y-start"),
	SpanYEnd: atom!("span-y-end"),
	YSelfStart: atom!("y-self-start"),
	YSelfEnd: atom!("y-self-end"),
	SpanYSelfStart: atom!("span-y-self-start"),
	SpanYSelfEnd: atom!("span-y-self-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaBlock {
	BlockStart: atom!("block-start"),
	Center: atom!("center"),
	BlockEnd: atom!("block-end"),
	SpanBlockStart: atom!("span-block-start"),
	SpanBlockEnd: atom!("span-block-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaInline {
	InlineStart: atom!("inline-start"),
	Center: atom!("center"),
	InlineEnd: atom!("inline-end"),
	SpanInlineStart: atom!("span-inline-start"),
	SpanInlineEnd: atom!("span-inline-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaSelfBlock {
	SelfBlockStart: atom!("self-block-start"),
	Center: atom!("center"),
	SelfBlockEnd: atom!("self-block-end"),
	SpanSelfBlockStart: atom!("span-self-block-start"),
	SpanSelfBlockEnd: atom!("span-self-block-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaSelfInline {
	SelfInlineStart: atom!("self-inline-start"),
	Center: atom!("center"),
	SelfInlineEnd: atom!("self-inline-end"),
	SpanSelfInlineStart: atom!("span-self-inline-start"),
	SpanSelfInlineEnd: atom!("span-self-inline-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaPosition {
	Start: atom!("start"),
	Center: atom!("center"),
	End: atom!("end"),
	SpanStart: atom!("span-start"),
	SpanEnd: atom!("span-end"),
	SpanAll: atom!("span-all"),
});

keyword_typedef!(PositionAreaSelfPosition {
	SelfStart: atom!("self-start"),
	Center: atom!("center"),
	SelfEnd: atom!("self-end"),
	SpanSelfStart: atom!("span-self-start"),
	SpanSelfEnd: atom!("span-self-end"),
	SpanAll: atom!("span-all"),
});
