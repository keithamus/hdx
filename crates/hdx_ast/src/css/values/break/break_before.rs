#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-break-before
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BreakBefore {
	#[default]
	Auto, // atom!("auto")
	Avoid,       // atom!("avoid")
	Always,      // atom!("always")
	All,         // atom!("all")
	AvoidPage,   // atom!("avoid-page")
	Page,        // atom!("page")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Recto,       // atom!("recto")
	Verso,       // atom!("verso")
	AvoidColumn, // atom!("avoid-column")
	Column,      // atom!("column")
	AvoidRegion, // atom!("avoid-region")
	Region,      // atom!("region")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<BreakBefore>(), 1);
	}
}
