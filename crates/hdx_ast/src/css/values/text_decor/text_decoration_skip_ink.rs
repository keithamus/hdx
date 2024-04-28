use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextDecorationSkipInk {
	#[default]
	Auto, // atom!("auto")
	None, // atom!("none")
	All,  // atom!("all")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextDecorationSkipInk, 1);
	}
}
