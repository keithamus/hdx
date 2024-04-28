use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::CSSInt;

#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum ZIndex {
	#[default]
	Auto,
	#[parsable(Number)]
	Integer(CSSInt),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ZIndex, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ZIndex, "auto");
		assert_parse!(ZIndex, "999");
	}
}
