pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

// use super::types::LengthPercentage;
use super::{MarginTop, PaddingTop};

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
		assert_size!(MarginTop, 16);
		assert_size!(MarginRight, 16);
		assert_size!(MarginBottom, 16);
		assert_size!(MarginLeft, 16);
		assert_size!(Margin, 64);
		assert_size!(PaddingTop, 12);
		assert_size!(PaddingRight, 12);
		assert_size!(PaddingBottom, 12);
		assert_size!(PaddingLeft, 12);
		assert_size!(Padding, 48);
		// assert_size!(MarginTrim, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MarginLeft, "auto");
		assert_parse!(Margin, "1px 1px");
		assert_parse!(Margin, "1px 2px");
		assert_parse!(Margin, "1px 2px 3px 4px");
	}
}
