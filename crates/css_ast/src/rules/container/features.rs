use crate::{properties::Property, types::Ratio, units::Length, Visit, Visitable};
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	discrete_feature, keyword_set, ranged_feature, ConditionKeyword, CursorSink, FeatureConditionList, Parse, Parser,
	Peek, RangedFeatureKeyword, Result as ParserResult, ToCursors,
};
use hdx_proc_macro::visit;

keyword_set!(WidthContainerFeatureKeyword { Width: "width" });
impl RangedFeatureKeyword for WidthContainerFeatureKeyword {}

#[visit]
ranged_feature!(WidthContainerFeature, WidthContainerFeatureKeyword, Length);

impl<'a> Visitable<'a> for WidthContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_width_container_feature(self);
	}
}

keyword_set!(HeightContainerFeatureKeyword { Height: "height" });
impl RangedFeatureKeyword for HeightContainerFeatureKeyword {}

#[visit]
ranged_feature!(HeightContainerFeature, HeightContainerFeatureKeyword, Length);

impl<'a> Visitable<'a> for HeightContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_height_container_feature(self);
	}
}

keyword_set!(InlineSizeContainerFeatureKeyword { InlineSize: "inline-size" });
impl RangedFeatureKeyword for InlineSizeContainerFeatureKeyword {}

#[visit]
ranged_feature!(InlineSizeContainerFeature, InlineSizeContainerFeatureKeyword, Length);

impl<'a> Visitable<'a> for InlineSizeContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_inline_size_container_feature(self);
	}
}

keyword_set!(BlockSizeContainerFeatureKeyword { BlockSize: "block-size" });
impl RangedFeatureKeyword for BlockSizeContainerFeatureKeyword {}

#[visit]
ranged_feature!(BlockSizeContainerFeature, BlockSizeContainerFeatureKeyword, Length);

impl<'a> Visitable<'a> for BlockSizeContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_block_size_container_feature(self);
	}
}

keyword_set!(AspectRatioContainerFeatureKeyword { AspectRatio: "aspect-ratio" });
impl RangedFeatureKeyword for AspectRatioContainerFeatureKeyword {}

#[visit]
ranged_feature!(AspectRatioContainerFeature, AspectRatioContainerFeatureKeyword, Ratio);

impl<'a> Visitable<'a> for AspectRatioContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_aspect_ratio_container_feature(self);
	}
}

keyword_set!(OrientationContainerFeatureKeyword { Portrait: "portrait", Landscape: "landscape" });

#[visit]
discrete_feature!(OrientationContainerFeature, "orientation", OrientationContainerFeatureKeyword);

impl<'a> Visitable<'a> for OrientationContainerFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_orientation_container_feature(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum StyleQuery<'a> {
	Is(Property<'a>),
	Not(ConditionKeyword, Property<'a>),
	And(Vec<'a, (Property<'a>, Option<ConditionKeyword>)>),
	Or(Vec<'a, (Property<'a>, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for StyleQuery<'a> {
	type FeatureCondition = Property<'a>;
	fn build_is(feature: Property<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: Property<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (Property<'a>, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (Property<'a>, Option<ConditionKeyword>)>) -> Self {
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
			Self::Not(keyword, c) => {
				ToCursors::to_cursors(keyword, s);
				ToCursors::to_cursors(c, s);
			}
			Self::And(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
			Self::Or(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
		}
	}
}

impl<'a> Visitable<'a> for StyleQuery<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => Visitable::accept(feature, v),
			Self::Not(_, feature) => Visitable::accept(feature, v),
			Self::And(features) => {
				for (feature, _) in features {
					Visitable::accept(feature, v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
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
	Not(ConditionKeyword, ScrollStateFeature),
	And(Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>),
	Or(Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for ScrollStateQuery<'a> {
	type FeatureCondition = ScrollStateFeature;
	fn build_is(feature: ScrollStateFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: ScrollStateFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>) -> Self {
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
			Self::Not(keyword, c) => {
				ToCursors::to_cursors(keyword, s);
				ToCursors::to_cursors(c, s);
			}
			Self::And(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
			Self::Or(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
		}
	}
}

impl<'a> Visitable<'a> for ScrollStateQuery<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => Visitable::accept(feature, v),
			Self::Not(_, feature) => Visitable::accept(feature, v),
			Self::And(features) => {
				for (feature, _) in features {
					Visitable::accept(feature, v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
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

keyword_set!(ScrollStateFeatureKeyword { Scrollable: "scrollable", Snapped: "snapped", Stuck: "stuck" });

impl<'a> Peek<'a> for ScrollStateFeature {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		ScrollStateFeatureKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for ScrollStateFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let keyword = p.parse::<ScrollStateFeatureKeyword>()?;
		match keyword {
			ScrollStateFeatureKeyword::Scrollable(_) => p.parse::<ScrollableScrollStateFeature>().map(Self::Scrollable),
			ScrollStateFeatureKeyword::Snapped(_) => p.parse::<SnappedScrollStateFeature>().map(Self::Snapped),
			ScrollStateFeatureKeyword::Stuck(_) => p.parse::<StuckScrollStateFeature>().map(Self::Stuck),
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
discrete_feature!(ScrollableScrollStateFeature, "scrollable", ScrollableScrollStateFeatureKeyword);

keyword_set!(ScrollableScrollStateFeatureKeyword {
	None: "none",
	Top: "top",
	Right: "right",
	Bottom: "bottom",
	Left: "left",
	BlockStart: "block-start",
	InlineStart: "inline-start",
	BlockEnd: "block-end",
	InlineEnd: "inline-end",
	X: "x",
	Y: "y",
	Block: "block",
	Inline: "inline",
	Discrete: "discrete",
});

impl<'a> Visitable<'a> for ScrollableScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_scrollable_scroll_state_feature(self);
	}
}

#[visit]
discrete_feature!(SnappedScrollStateFeature, "snapped", SnappedScrollStateFeatureKeyword);

keyword_set!(SnappedScrollStateFeatureKeyword {
	None: "none",
	X: "x",
	Y: "y",
	Block: "block",
	Inline: "inline",
	Both: "both",
	Discrete: "discrete",
});

impl<'a> Visitable<'a> for SnappedScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_snapped_scroll_state_feature(self);
	}
}

#[visit]
discrete_feature!(StuckScrollStateFeature, "stuck", StuckScrollStateFeatureKeyword);

keyword_set!(StuckScrollStateFeatureKeyword {
	None: "none",
	Top: "top",
	Right: "right",
	Bottom: "bottom",
	Left: "left",
	BlockStart: "block-start",
	InlineStart: "inline-start",
	BlockEnd: "block-end",
	InlineEnd: "inline-end",
	Discrete: "discrete",
});

impl<'a> Visitable<'a> for StuckScrollStateFeature {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_stuck_scroll_state_feature(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<HeightContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<InlineSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<BlockSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<AspectRatioContainerFeature>(), 188);
		assert_eq!(std::mem::size_of::<OrientationContainerFeature>(), 64);
		assert_eq!(std::mem::size_of::<StyleQuery>(), 408);
		assert_eq!(std::mem::size_of::<ScrollStateQuery>(), 88);
		assert_eq!(std::mem::size_of::<ScrollStateFeature>(), 68);
		assert_eq!(std::mem::size_of::<ScrollableScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<SnappedScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<StuckScrollStateFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthContainerFeature, "(width:360px)");
		assert_parse!(WidthContainerFeature, "(width>=1400px)");
		assert_parse!(WidthContainerFeature, "(100px<=width)");
		assert_parse!(WidthContainerFeature, "(100px<=width>1400px)");
		assert_parse!(HeightContainerFeature, "(height:360px)");
		assert_parse!(HeightContainerFeature, "(height>=1400px)");
		assert_parse!(HeightContainerFeature, "(100px<=height)");
		assert_parse!(HeightContainerFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WidthContainerFeature, "(min-width > 10px)");
		assert_parse_error!(WidthContainerFeature, "(width: 1%)");
	}
}
