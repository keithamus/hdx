use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-white-space-collapse
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WhiteSpaceCollapse {
	#[default]
	Collapse, // atom!("collapse")
	Discard,        // atom!("discard")
	Preserve,       // atom!("preserve")
	PreserveBreaks, // atom!("preserve-breaks")
	PreserveSpaces, // atom!("preserve-spaces")
	BreakSpaces,    // atom!("break-spaces")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WhiteSpaceCollapse, 1);
	}
}
