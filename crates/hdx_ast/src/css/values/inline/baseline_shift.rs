#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Parsable, Value, Writable, css::values::units::LengthPercentage};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Value, Parsable, Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BaselineShift {
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
	Sub,    // atom!("sub")
	Super,  // atom!("super")
	Top,    // atom!("top")
	Center, // atom!("center")
	Bottom, // atom!("bottom")
}

impl Default for BaselineShift {
    fn default() -> Self {
		Self::LengthPercentage(LengthPercentage::Zero)
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BaselineShift, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BaselineShift, "sub");
		assert_parse!(BaselineShift, "0");
		assert_parse!(BaselineShift, "200px");
	}
}
