#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Parsable, Value, Writable};

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Visibility {
	#[default]
	Visible, // atom!("visible"),
	Hidden,   // atom!("hidden"),
	Collapse, // atom!("collapse"),
}

impl Value for Visibility {
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
		assert_eq!(size_of::<Visibility>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Visibility>(&allocator, "visible", "visible");
	}
}
