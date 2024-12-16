use hdx_parser::keyword_typedef;

pub(crate) use crate::css::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::OverflowBlockStyleValue;

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// For convenience, the following value types are defined to represents commonly used subsets of <box>:
// <visual-box> = content-box | padding-box | border-box
keyword_typedef!(VisualBox {
	ContentBox: atom!("content-box"),
	PaddingBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
});
