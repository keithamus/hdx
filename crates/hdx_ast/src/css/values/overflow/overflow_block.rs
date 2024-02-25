#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-overflow-3/#propdef-overflow-block
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum OverflowBlock {
	#[default]
	Visible, // atom!("visible")
	Hidden, // atom!("hidden")
	Clip,   // atom!("clip")
	Scroll, // atom!("scroll")
	Auto,   // atom!("auto")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<OverflowBlock>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<OverflowBlock>(&allocator, "clip", "clip");
	}
}
