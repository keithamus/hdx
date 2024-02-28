#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum ZIndex {
	#[default]
	Auto,
	#[parsable(Number, Check::Int)]
	Integer(CSSFloat),
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ZIndex>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<ZIndex>(&allocator, "auto", "auto");
		test_write::<ZIndex>(&allocator, "999", "999");
	}
}
