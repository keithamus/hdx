pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

// use super::types::LengthPercentage;
use super::{MarginTopStyleValue, PaddingTopStyleValue};

// shortcuts for logical properties to resolve to 0
// impl MarginTop {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: MarginTop = MarginTop::LengthPercentage(LengthPercentage::Zero);
// }

// impl PaddingTop {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: PaddingTop = PaddingTop(LengthPercentage::Zero);
// }

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(MarginTopStyleValue, 16);
		assert_size!(MarginRightStyleValue, 16);
		assert_size!(MarginBottomStyleValue, 16);
		assert_size!(MarginLeftStyleValue, 16);
		assert_size!(MarginStyleValue, 64);
		assert_size!(PaddingTopStyleValue, 12);
		assert_size!(PaddingRightStyleValue, 12);
		assert_size!(PaddingBottomStyleValue, 12);
		assert_size!(PaddingLeftStyleValue, 12);
		assert_size!(PaddingStyleValue, 48);
		// assert_size!(MarginTrimStyleValue, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MarginLeftStyleValue, "auto");
		assert_parse!(MarginStyleValue, "1px 1px");
		assert_parse!(MarginStyleValue, "1px 2px");
		assert_parse!(MarginStyleValue, "1px 2px 3px 4px");
	}
}
