use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-tables-3/#propdef-border-collapse
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BorderCollapse {
	#[default]
	Separate, // atom!("separate")
	Collapse, // atom!("collapse")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderCollapse, 1);
	}
}
