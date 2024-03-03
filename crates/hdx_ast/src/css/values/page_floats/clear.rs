use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-page-floats-3/#propdef-clear
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Clear {
	InlineStart, // atom!("inline-start")
	InlineEnd,   // atom!("inline-end")
	BlockStart,  // atom!("block-start")
	BlockEnd,    // atom!("block-end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Top,         // atom!("top")
	Bottom,      // atom!("bottom")
	BothInline,  // atom!("both-inline")
	BothBlock,   // atom!("both-block")
	Both,        // atom!("both")
	#[default]
	None, // atom!("none")
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Clear, 1);
	}
}
