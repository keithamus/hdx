#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontWeight {
	#[parsable(Number, Check::Range(1.0..=1000.0), Check::Int)]
	Number(CSSFloat),
	#[default]	
	Normal,  // atom!("normal")
	Bold,    // atom!("bold")
	Bolder,  // atom!("bolder")
	Lighter, // atom!("lighter")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<FontWeight>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<FontWeight>(&allocator, "normal", "normal");
		test_write::<FontWeight>(&allocator, "1", "1");
		test_write::<FontWeight>(&allocator, "1000", "1000");
	}
}
