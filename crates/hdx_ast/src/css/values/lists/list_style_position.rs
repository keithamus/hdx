#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Value, Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ListStylePosition {
	#[default]
	Outside, // atom!("outside")
	Inside, // atom!("inside")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ListStylePosition, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ListStylePosition, "inside");
		assert_parse!(ListStylePosition, "outside");
	}
}
