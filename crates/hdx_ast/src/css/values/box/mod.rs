#[cfg(feature = "serde")]
use serde::Serialize;

mod margin_trim;

pub use margin_trim::*;

use super::Todo;
use crate::{css::values::units::LengthPercentage, Parsable, Value, Writable};

// https://drafts.csswg.org/css-box-4

// https://drafts.csswg.org/css-box-4/#padding-physical
pub type Padding = Todo;

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingTop(#[parsable(FromToken)] pub LengthPercentage);

impl Value for PaddingTop {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingRight(#[parsable(FromToken)] pub LengthPercentage);

impl Value for PaddingRight {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingBottom(#[parsable(FromToken)] pub LengthPercentage);

impl Value for PaddingBottom {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingLeft(#[parsable(FromToken)] pub LengthPercentage);

impl Value for PaddingLeft {}

// https://drafts.csswg.org/css-box-4/#margin-physical
pub type Margin = Todo;

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginTop {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

impl Value for MarginTop {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginRight {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

impl Value for MarginRight {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginBottom {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

impl Value for MarginBottom {}

#[derive(Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginLeft {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

impl Value for MarginLeft {}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<PaddingTop>(), 8);
		assert_eq!(size_of::<PaddingRight>(), 8);
		assert_eq!(size_of::<PaddingLeft>(), 8);
		assert_eq!(size_of::<PaddingBottom>(), 8);
		assert_eq!(size_of::<MarginTop>(), 8);
		assert_eq!(size_of::<MarginRight>(), 8);
		assert_eq!(size_of::<MarginLeft>(), 8);
		assert_eq!(size_of::<MarginBottom>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<PaddingTop>(&allocator, "1px", "1px");
		test_write::<PaddingTop>(&allocator, "100%", "100%");
		test_write::<MarginLeft>(&allocator, "auto", "auto");
	}
}
