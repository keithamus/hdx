use hdx_derive::{Atomizable, Parsable, Peekable, Writable};

pub(crate) use crate::css::units::*;

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// For convenience, the following value types are defined to represents commonly used subsets of <box>:
// <visual-box> = content-box | padding-box | border-box
#[derive(Atomizable, Writable, Parsable, Peekable, Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum VisualBox {
	ContentBox, // atom!("content-box")
	#[default]
	PaddingBox, // atom!("padding-box")
	BorderBox,  // atom!("border-box")
}
