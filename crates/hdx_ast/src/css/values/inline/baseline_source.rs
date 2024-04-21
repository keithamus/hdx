use crate::{Atomizable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[atomizable(FromToken)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BaselineSource {
	#[default]
	Auto, // atom!("auto")
	First, // atom!("first")
	Last,  // atom!("last")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BaselineSource, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BaselineSource, "auto");
		assert_parse!(BaselineSource, "first");
		assert_parse!(BaselineSource, "last");
	}
}
