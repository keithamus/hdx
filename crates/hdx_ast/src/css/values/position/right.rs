#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentage, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Right {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, FromToken)]
	LengthPercentage(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Right, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Right, "-10px");
		assert_parse!(Right, "auto");
	}
}
