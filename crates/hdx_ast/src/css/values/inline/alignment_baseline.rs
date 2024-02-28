#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum AlignmentBaseline {
	#[default]
	Baseline, // atom!("baseline")
	TextBottom,   // atom!("text-bottom")
	Alphabetic,   // atom!("alphabetic")
	Ideographic,  // atom!("ideographic")
	Middle,       // atom!("middle")
	Central,      // atom!("central")
	Mathematical, // atom!("mathematical")
	TextTop,      // atom!("text-top")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<AlignmentBaseline>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AlignmentBaseline>(&allocator, "baseline", "baseline");
		test_write::<AlignmentBaseline>(&allocator, "text-bottom", "text-bottom");
	}
}
