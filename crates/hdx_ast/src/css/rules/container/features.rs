use crate::css::units::Length;
use crate::css::{properties::Property, types::Ratio};
use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Kind;
use hdx_parser::{
	diagnostics, discrete_feature, keyword_typedef, ranged_feature, AtRule, Build, ConditionalAtRule, CursorSink, Is,
	Parse, Parser, Peek, PreludeList, Result as ParserResult, RuleList, ToCursors, T,
};

ranged_feature!(WidthContainerFeature[atom!("width")], Length);

ranged_feature!(HeightContainerFeature[atom!("height")], Length);

ranged_feature!(InlineSizeContainerFeature[atom!("inline-size")], Length);

ranged_feature!(BlockSizeContainerFeature[atom!("block-size")], Length);

ranged_feature!(AspectRatioContainerFeature[atom!("aspect-ratio")], Ratio);

discrete_feature!(OrientationContainerFeature[atom!("orientation")] {
	Portrait: atom!("portrait"),
	Landscape: atom!("landscape"),
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum StyleQuery<'a> {
	Is(Property<'a>),
	Not(Box<StyleQuery<'a>>),
	And(Vec<'a, Property<'a>>),
	Or(Vec<'a, Property<'a>>),
}

impl<'a> ConditionalAtRule<'a> for StyleQuery<'a> {
	type Feature = Property<'a>;
	fn new_is(feature: Property<'a>) -> Self {
		Self::Is(feature)
	}
	fn new_not(condition: StyleQuery<'a>) -> Self {
		Self::Not(Box::new(condition))
	}
	fn new_and(feature: Vec<'a, Property<'a>>) -> Self {
		Self::And(feature)
	}
	fn new_or(feature: Vec<'a, Property<'a>>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for StyleQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

impl<'a> ToCursors for StyleQuery<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Not(c) => ToCursors::to_cursors(c.as_ref(), s),
			Self::And(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
			Self::Or(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum ScrollStateQuery<'a> {
	Is(ScrollStateFeature),
	Not(Box<ScrollStateQuery<'a>>),
	And(Vec<'a, ScrollStateFeature>),
	Or(Vec<'a, ScrollStateFeature>),
}

impl<'a> ConditionalAtRule<'a> for ScrollStateQuery<'a> {
	type Feature = ScrollStateFeature;
	fn new_is(feature: ScrollStateFeature) -> Self {
		Self::Is(feature)
	}
	fn new_not(condition: ScrollStateQuery<'a>) -> Self {
		Self::Not(Box::new(condition))
	}
	fn new_and(feature: Vec<'a, ScrollStateFeature>) -> Self {
		Self::And(feature)
	}
	fn new_or(feature: Vec<'a, ScrollStateFeature>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for ScrollStateQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

impl<'a> ToCursors for ScrollStateQuery<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Not(c) => ToCursors::to_cursors(c.as_ref(), s),
			Self::And(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
			Self::Or(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ScrollStateFeature {
	Scrollable(ScrollableScrollStateFeature),
	Snapped(SnappedScrollStateFeature),
	Stuck(StuckScrollStateFeature),
}

impl<'a> Peek<'a> for ScrollStateFeature {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<T![Ident]>()
	}
}

impl<'a> Parse<'a> for ScrollStateFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		if c != Kind::Ident {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		match p.parse_atom_lower(c) {
			atom!("scrollable") => p.parse::<ScrollableScrollStateFeature>().map(Self::Scrollable),
			atom!("snapped") => p.parse::<SnappedScrollStateFeature>().map(Self::Snapped),
			atom!("stuck") => p.parse::<StuckScrollStateFeature>().map(Self::Stuck),
			atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors for ScrollStateFeature {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Scrollable(feature) => ToCursors::to_cursors(feature, s),
			Self::Snapped(feature) => ToCursors::to_cursors(feature, s),
			Self::Stuck(feature) => ToCursors::to_cursors(feature, s),
		}
	}
}

discrete_feature!(ScrollableScrollStateFeature[atom!("scrollable")] {
	None: atom!("none"),
	Top: atom!("top"),
	Right: atom!("right"),
	Bottom: atom!("bottom"),
	Left: atom!("left"),
	BlockStart: atom!("block-start"),
	InlineStart: atom!("inline-start"),
	BlockEnd: atom!("block-end"),
	InlineEnd: atom!("inline-end"),
	X: atom!("x"),
	Y: atom!("y"),
	Block: atom!("block"),
	Inline: atom!("inline"),
	Discrete: atom!("discrete"),
});

discrete_feature!(SnappedScrollStateFeature[atom!("snapped")] {
	None: atom!("none"),
	X: atom!("x"),
	Y: atom!("y"),
	Block: atom!("block"),
	Inline: atom!("inline"),
	Both: atom!("both"),
	Discrete: atom!("discrete"),
});

discrete_feature!(StuckScrollStateFeature[atom!("stuck")] {
	None: atom!("none"),
	Top: atom!("top"),
	Right: atom!("right"),
	Bottom: atom!("bottom"),
	Left: atom!("left"),
	BlockStart: atom!("block-start"),
	InlineStart: atom!("inline-start"),
	BlockEnd: atom!("block-end"),
	InlineEnd: atom!("inline-end"),
	Discrete: atom!("discrete"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WidthContainerFeature, 76);
		assert_size!(HeightContainerFeature, 76);
		assert_size!(InlineSizeContainerFeature, 76);
		assert_size!(BlockSizeContainerFeature, 76);
		assert_size!(AspectRatioContainerFeature, 124);
		assert_size!(OrientationContainerFeature, 36);
		assert_size!(StyleQuery, 360);
		assert_size!(ScrollStateQuery, 40);
		assert_size!(ScrollStateFeature, 40);
		assert_size!(ScrollableScrollStateFeature, 36);
		assert_size!(SnappedScrollStateFeature, 36);
		assert_size!(StuckScrollStateFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthContainerFeature, "width:360px");
		assert_parse!(WidthContainerFeature, "width>=1400px");
		assert_parse!(WidthContainerFeature, "100px<=width");
		assert_parse!(WidthContainerFeature, "100px<=width>1400px");
		assert_parse!(HeightContainerFeature, "height:360px");
		assert_parse!(HeightContainerFeature, "height>=1400px");
		assert_parse!(HeightContainerFeature, "100px<=height");
		assert_parse!(HeightContainerFeature, "100px<=height>1400px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WidthContainerFeature, "min-width > 10px");
		assert_parse_error!(WidthContainerFeature, "width: 1%");
	}
}
