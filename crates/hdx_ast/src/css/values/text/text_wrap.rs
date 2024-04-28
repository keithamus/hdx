use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-text-wrap
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextWrap {
	#[default]
	Wrap, // atom!("wrap")
	Nowrap,  // atom!("nowrap")
	Balance, // atom!("balance")
	Stable,  // atom!("stable")
	Pretty,  // atom!("pretty")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextWrap, 1);
	}
}
