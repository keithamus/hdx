use css_parse::keyword_set;

pub(crate) use crate::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::{
	ColumnGapStyleValue,
	/*AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf, */ RowGapStyleValue,
};

// https://drafts.csswg.org/css-align-3/#typedef-baseline-position
keyword_set!(BaselinePosition { First: "first", Last: "last", Baseline: "baseline" });

// https://drafts.csswg.org/css-align-3/#typedef-overflow-position
keyword_set!(OverflowPosition { Unsafe: "unsafe", Safe: "safe" });

// https://drafts.csswg.org/css-align-3/#typedef-self-position
keyword_set!(SelfPosition {
	Center: "center",
	Start: "start",
	End: "end",
	SelfStart: "self-start",
	SelfEnd: "self-end",
	FlexStart: "flex-start",
	FlexEnd: "flex-end",
});
