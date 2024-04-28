use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-sizing/#propdef-box-sizing
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxSizing {
	#[default]
	ContentBox, // atom!("content-box")
	BorderBox, // atom!("border-box")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BoxSizing, 1);
	}
}
