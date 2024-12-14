pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

// use super::types::LengthPercentage;
// use super::{MinWidthStyleValue, Width};

// shortcuts for logical properties to resolve to 0
// impl Width {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: Width = Width::LengthPercentage(LengthPercentage::Zero();
// }
//
// impl MinWidth {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: MinWidth = MinWidth::LengthPercentage(LengthPercentage::Zero);
// }

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WidthStyleValue, 36);
		assert_size!(HeightStyleValue, 36);
		assert_size!(MinWidthStyleValue, 36);
		assert_size!(MinHeightStyleValue, 36);
		assert_size!(MaxWidthStyleValue, 36);
		assert_size!(MaxHeightStyleValue, 36);
		assert_size!(BoxSizingStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthStyleValue, "0");
		assert_parse!(WidthStyleValue, "1px");
		assert_parse!(WidthStyleValue, "fit-content");
		assert_parse!(WidthStyleValue, "fit-content(20rem)");
		assert_parse!(WidthStyleValue, "fit-content(0)");
	}
}
