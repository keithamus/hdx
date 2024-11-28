use hdx_parser::keyword_typedef;

pub(crate) use crate::css::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::{
	ColumnGap, /*AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf, */ RowGap,
};

// https://drafts.csswg.org/css-align-3/#typedef-baseline-position
keyword_typedef!(BaselinePosition { First: atom!("first"), Last: atom!("last"), Baseline: atom!("baseline") });

// https://drafts.csswg.org/css-align-3/#typedef-overflow-position
keyword_typedef!(OverflowPosition { Unsafe: atom!("unsafe"), Safe: atom!("safe") });

// https://drafts.csswg.org/css-align-3/#typedef-self-position
keyword_typedef!(SelfPosition {
	Center: atom!("center"),
	Start: atom!("start"),
	End: atom!("end"),
	SelfStart: atom!("self-start"),
	SelfEnd: atom!("self-end"),
	FlexStart: atom!("flex-start"),
	FlexEnd: atom!("flex-end"),
});
