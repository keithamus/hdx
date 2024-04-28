use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-flexbox/#flex-wrap-property
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FlexWrap {
	#[default]
	NoWrap, // atom!("no-wrap")
	Wrap,        // atom!("wrap")
	WrapReverse, // atom!("wrap-reverse")
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FlexWrap, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FlexWrap, "wrap-reverse");
	}
}
