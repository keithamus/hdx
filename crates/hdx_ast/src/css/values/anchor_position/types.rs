use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

pub(crate) use crate::css::types::DashedIdent;
use crate::macros::keyword_typedef;

pub type AnchorName = DashedIdent;

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
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PositionArea {
	Physical(PositionAreaPhsyicalHorizontal, PositionAreaPhsyicalVertical),
	Logical(PositionAreaBlock, PositionAreaInline),
	SelfLogical(PositionAreaSelfBlock, PositionAreaSelfInline),
	Position(PositionAreaPosition, PositionAreaPosition),
	SelfPosition(PositionAreaSelfPosition, PositionAreaSelfPosition),
}

impl<'a> Peek<'a> for PositionArea {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<PositionAreaPhsyicalVertical>()
			.or_else(|| p.peek::<PositionAreaPhsyicalHorizontal>())
			.or_else(|| p.peek::<PositionAreaBlock>())
			.or_else(|| p.peek::<PositionAreaInline>())
			.or_else(|| p.peek::<PositionAreaSelfBlock>())
			.or_else(|| p.peek::<PositionAreaSelfInline>())
			.or_else(|| p.peek::<PositionAreaPosition>())
			.or_else(|| p.peek::<PositionAreaSelfPosition>())
	}
}

impl<'a> Parse<'a> for PositionArea {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(first) = p.parse_if_peek::<PositionAreaPosition>()? {
			let second =
				if let Some(token) = p.parse_if_peek::<PositionAreaPosition>()? { token } else { first.clone() };
			Ok(Self::Position(first, second))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaSelfPosition>()? {
			let second =
				if let Some(token) = p.parse_if_peek::<PositionAreaSelfPosition>()? { token } else { first.clone() };
			Ok(Self::SelfPosition(first, second))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaBlock>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaInline>()? {
				token
			} else {
				PositionAreaInline::SpanAll
			};
			Ok(Self::Logical(first, second))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaInline>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaBlock>()? {
				token
			} else {
				PositionAreaBlock::SpanAll
			};
			Ok(Self::Logical(second, first))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaSelfBlock>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaSelfInline>()? {
				token
			} else {
				PositionAreaSelfInline::SpanAll
			};
			Ok(Self::SelfLogical(first, second))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaSelfInline>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaSelfBlock>()? {
				token
			} else {
				PositionAreaSelfBlock::SpanAll
			};
			Ok(Self::SelfLogical(second, first))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaPhsyicalVertical>()? {
				token
			} else {
				PositionAreaPhsyicalVertical::SpanAll
			};
			Ok(Self::Physical(first, second))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaPhsyicalVertical>()? {
			let second = if let Some(token) = p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()? {
				token
			} else {
				PositionAreaPhsyicalHorizontal::SpanAll
			};
			Ok(Self::Physical(second, first))
		} else {
			let token = p.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}

impl<'a> WriteCss<'a> for PositionArea {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Physical(h, v) => write_css!(sink, h, ' ', v),
			Self::Logical(b, i) => write_css!(sink, b, ' ', i),
			Self::SelfLogical(b, i) => write_css!(sink, b, ' ', i),
			Self::Position(p1, p2) => write_css!(sink, p1, ' ', p2),
			Self::SelfPosition(p1, p2) => write_css!(sink, p1, ' ', p2),
		}
		Ok(())
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
