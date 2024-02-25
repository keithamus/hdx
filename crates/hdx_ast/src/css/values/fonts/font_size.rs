#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentage, Parsable, Value, Writable};

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontSize {
	XxSmall, // atom!("xx-small")
	XSmall, // atom!("x-small")
	Small, // atom!("small")
	#[default]
	Medium, // atom!("medium")
	Large, // atom!("large")
	XLarge, // atom!("x-large")
	XxLarge, // atom!("xx-large")
	Larger, // atom!("larger")
	Smaller, // atom!("smaller")
	#[parsable(DimensionOrZero, FromToken, Check::Positive)]
	LengthPercentage(LengthPercentage)
}

impl Value for FontSize {
	fn inherits() -> bool {
		true
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
		assert_eq!(size_of::<FontSize>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<FontSize>(&allocator, "10px", "10px");
		test_write::<FontSize>(&allocator, "xx-large", "xx-large");
	}
}
