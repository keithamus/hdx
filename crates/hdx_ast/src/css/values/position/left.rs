#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentage, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Left {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Left>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Left>(&allocator, "-10px", "-10px");
		test_write::<Left>(&allocator, "auto", "auto");
	}
}
