pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

use super::types::LengthPercentage;
use super::{MinWidth, Width};

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
		assert_size!(Width, 36);
		assert_size!(Height, 36);
		assert_size!(MinWidth, 36);
		assert_size!(MinHeight, 36);
		assert_size!(MaxWidth, 36);
		assert_size!(MaxHeight, 36);
		assert_size!(BoxSizing, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Width, "0");
		assert_parse!(Width, "1px");
		assert_parse!(Width, "fit-content");
		assert_parse!(Width, "fit-content(20rem)");
		assert_parse!(Width, "fit-content(0)");
	}
}
