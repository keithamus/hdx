use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-break-before
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
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
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BreakBefore, 1);
	}
}
