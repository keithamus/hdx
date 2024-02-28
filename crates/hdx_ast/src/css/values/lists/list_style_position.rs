#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Value, Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ListStylePosition {
	#[default]
	Outside, // atom!("outside")
	Inside, // atom!("inside")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ListStylePosition>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<ListStylePosition>(&allocator, "inside", "inside");
		test_write::<ListStylePosition>(&allocator, "outside", "outside");
	}
}
