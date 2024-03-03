use crate::{css::values::units::CSSFloat, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum ZIndex {
	#[default]
	Auto,
	#[parsable(Number, Check::Int)]
	Integer(CSSFloat),
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
