#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-position-3/#propdef-position
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WritingMode {
	#[default]
	HorizontalTb, // atom!("horizontal-tb")
	VerticalRl, // atom!("vertical-rl")
	VerticalLr, // atom!("vertical-lr")
	SidewaysRl, // atom!("sideways-rl")
	SidewaysLr, // atom!("sideways-lr")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WritingMode, 1);
	}
}
