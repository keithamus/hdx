use crate::{css::units::Percent, Parsable, Writable};

// https://drafts.csswg.org/css-size-adjust-1/#propdef-text-size-adjust
#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TextSizeAdjust {
	None,
	#[default]
	Auto,
	#[parsable(Dimension, Check::Range(0.0..), atom = "%")]
	Percentage(Percent),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextSizeAdjust, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextSizeAdjust, "0%");
		assert_parse!(TextSizeAdjust, "120%");
		assert_parse!(TextSizeAdjust, "auto");
	}
}
