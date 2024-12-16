use crate::css::{properties::Property, types::Ratio, units::Length, Visit, Visitable};
use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Kind;
use hdx_parser::{
	diagnostics, discrete_feature, ranged_feature, ConditionalAtRule, CursorSink, Parse, Parser, Peek,
	Result as ParserResult, ToCursors, T,
};
use hdx_proc_macro::visit;

#[visit]
ranged_feature!(WidthContainerFeature[atom!("width")], Length);

impl<'a> Visitable<'a> for WidthContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_width_container_feature(self);
	}
}

#[visit]
ranged_feature!(HeightContainerFeature[atom!("height")], Length);

impl<'a> Visitable<'a> for HeightContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_height_container_feature(self);
	}
}

#[visit]
ranged_feature!(InlineSizeContainerFeature[atom!("inline-size")], Length);

impl<'a> Visitable<'a> for InlineSizeContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_inline_size_container_feature(self);
	}
}

#[visit]
ranged_feature!(BlockSizeContainerFeature[atom!("block-size")], Length);

impl<'a> Visitable<'a> for BlockSizeContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_block_size_container_feature(self);
	}
}

#[visit]
ranged_feature!(AspectRatioContainerFeature[atom!("aspect-ratio")], Ratio);

impl<'a> Visitable<'a> for AspectRatioContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_aspect_ratio_container_feature(self);
	}
}

#[visit]
discrete_feature!(OrientationContainerFeature[atom!("orientation")] {
	Portrait: atom!("portrait"),
	Landscape: atom!("landscape"),
});

impl<'a> Visitable<'a> for OrientationContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_orientation_container_feature(self);
	}
}

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

impl ToCursors for StyleQuery<'_> {
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

impl<'a> Visitable<'a> for StyleQuery<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => Visitable::accept(feature, v),
			Self::Not(feature) => Visitable::accept(feature.as_ref(), v),
			Self::And(features) => {
				for feature in features {
					Visitable::accept(feature, v);
				}
			}
			Self::Or(features) => {
				for feature in features {
					Visitable::accept(feature, v);
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

impl ToCursors for ScrollStateQuery<'_> {
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

impl<'a> Visitable<'a> for ScrollStateQuery<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => Visitable::accept(feature, v),
			Self::Not(feature) => Visitable::accept(feature.as_ref(), v),
			Self::And(features) => {
				for feature in features {
					Visitable::accept(feature, v);
				}
			}
			Self::Or(features) => {
				for feature in features {
					Visitable::accept(feature, v);
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

impl ToCursors for ScrollStateFeature {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Scrollable(feature) => ToCursors::to_cursors(feature, s),
			Self::Snapped(feature) => ToCursors::to_cursors(feature, s),
			Self::Stuck(feature) => ToCursors::to_cursors(feature, s),
		}
	}
}

impl<'a> Visitable<'a> for ScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Scrollable(feature) => Visitable::accept(feature, v),
			Self::Snapped(feature) => Visitable::accept(feature, v),
			Self::Stuck(feature) => Visitable::accept(feature, v),
		}
	}
}

#[visit]
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

impl<'a> Visitable<'a> for ScrollableScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_scrollable_scroll_state_feature(self);
	}
}

#[visit]
discrete_feature!(SnappedScrollStateFeature[atom!("snapped")] {
	None: atom!("none"),
	X: atom!("x"),
	Y: atom!("y"),
	Block: atom!("block"),
	Inline: atom!("inline"),
	Both: atom!("both"),
	Discrete: atom!("discrete"),
});

impl<'a> Visitable<'a> for SnappedScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_snapped_scroll_state_feature(self);
	}
}

#[visit]
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

impl<'a> Visitable<'a> for StuckScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_stuck_scroll_state_feature(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WidthContainerFeature, 100);
		assert_size!(HeightContainerFeature, 100);
		assert_size!(InlineSizeContainerFeature, 100);
		assert_size!(BlockSizeContainerFeature, 100);
		assert_size!(AspectRatioContainerFeature, 164);
		assert_size!(OrientationContainerFeature, 40);
		assert_size!(StyleQuery, 400);
		assert_size!(ScrollStateQuery, 48);
		assert_size!(ScrollStateFeature, 44);
		assert_size!(ScrollableScrollStateFeature, 40);
		assert_size!(SnappedScrollStateFeature, 40);
		assert_size!(StuckScrollStateFeature, 40);
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
