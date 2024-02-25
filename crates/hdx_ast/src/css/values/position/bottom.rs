#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentage, Parsable, Value, Writable};

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Bottom {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

impl Value for Bottom {}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Bottom>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Bottom>(&allocator, "-10px", "-10px");
		test_write::<Bottom>(&allocator, "auto", "auto");
	}
}
