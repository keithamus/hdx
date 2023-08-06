#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-overflow-3/#propdef-overflow-block
#[derive(Atomizable, Copy, Clone, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum OverflowKeyword {
	#[default]
	Visible, // atom!("visible")
	Hidden, // atom!("hidden")
	Clip,   // atom!("clip")
	Scroll, // atom!("scroll")
	Auto,   // atom!("auto")
}
