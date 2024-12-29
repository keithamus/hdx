use css_parse::keyword_set;

pub(crate) use crate::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::OverflowBlockStyleValue;

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// For convenience, the following value types are defined to represents commonly used subsets of <box>:
// <visual-box> = content-box | padding-box | border-box
keyword_set!(VisualBox { ContentBox: "content-box", PaddingBox: "padding-box", BorderBox: "border-box" });
