#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::values::units::{CSSFloat, LengthPercentage},
	Parsable, Value, Writable,
};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LineHeight {
	#[default]
	Normal, // atom!("normal")
	#[parsable(Number, Check::Range(1.0..))]
	Number(CSSFloat),
	#[parsable(Dimension, FromToken, Check::Range(1.0..))]
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
		assert_eq!(size_of::<LineHeight>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<LineHeight>(&allocator, "10px", "10px");
		test_write::<LineHeight>(&allocator, "1.25", "1.25");
		test_write::<LineHeight>(&allocator, "normal", "normal");
	}
}
