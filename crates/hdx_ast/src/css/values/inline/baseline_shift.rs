#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Parsable, Value, Writable, css::values::units::LengthPercentage};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Value, Parsable, Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BaselineShift {
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
	Sub,    // atom!("sub")
	Super,  // atom!("super")
	Top,    // atom!("top")
	Center, // atom!("center")
	Bottom, // atom!("bottom")
}

impl Default for BaselineShift {
    fn default() -> Self {
		Self::LengthPercentage(LengthPercentage::Zero)
    }
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<BaselineShift>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<BaselineShift>(&allocator, "sub", "sub");
		test_write::<BaselineShift>(&allocator, "0", "0");
		test_write::<BaselineShift>(&allocator, "200px", "200px");
	}
}
