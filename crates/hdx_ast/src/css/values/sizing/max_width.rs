#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::LengthPercentage;
use crate::{Parsable, Writable, traits::Value};

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MaxWidth {
	#[default]
	None, // atom!("none")
	MinContent, // atom!("min-content")
	MaxContent, // atom!("max-content")  TODO: `intrinsic` non standard
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,    // atom!("stretch")  TODO: -webkit-fill-available, -moz-available
	FitContent, // atom!("fit-content")
	Contain,    // atom!("contain")

	#[parsable(DimensionOrZero, FromToken, Check::Positive)]
	LengthPercentage(LengthPercentage),
	#[parsable(Function, FromToken, Check::Positive, atom = "fit-content")]
	#[writable(as_function = "fit-content")]
	FitContentFunction(LengthPercentage),
}

impl Value for MaxWidth {
	fn initial() -> MaxWidth {
		MaxWidth::None
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
		assert_eq!(size_of::<MaxWidth>(), 12);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<MaxWidth>(&allocator, "0", "0");
		test_write::<MaxWidth>(&allocator, "1px", "1px");
		test_write::<MaxWidth>(&allocator, "none", "none");
		test_write::<MaxWidth>(&allocator, "fit-content", "fit-content");
		test_write::<MaxWidth>(&allocator, "fit-content(20rem)", "fit-content(20rem)");
		test_write::<MaxWidth>(&allocator, "fit-content(0)", "fit-content(0)");
	}
}
