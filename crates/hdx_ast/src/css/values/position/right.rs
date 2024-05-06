use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::LengthPercentage;

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Right {
	#[default]
	Auto, // atom!("auto"),
	#[parsable(DimensionOrZero, parse_inner)]
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
