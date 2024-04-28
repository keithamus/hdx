use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::CSSInt;

// https://drafts.csswg.org/css-break/#widows-orphans
#[derive(Value, Parsable, Writable, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Orphans(pub CSSInt);

impl Default for Orphans {
	fn default() -> Self {
		Self(2.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Orphans, 4);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Orphans, "8");
		assert_parse!(Orphans, "1");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Orphans, "8.2");
		assert_parse_error!(Orphans, "10%");
	}
}
