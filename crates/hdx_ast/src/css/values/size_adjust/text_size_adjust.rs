#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Parsable, Writable};

// https://drafts.csswg.org/css-size-adjust-1/#propdef-text-size-adjust
#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextSizeAdjust {
	None,
	#[default]
	Auto,
	#[writable(suffix = "%")]
	#[parsable(Dimension, atom = "%")]
	Percentage(CSSFloat),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextSizeAdjust, 8);
	}
}
