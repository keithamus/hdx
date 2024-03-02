#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-white-space-collapse
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
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
