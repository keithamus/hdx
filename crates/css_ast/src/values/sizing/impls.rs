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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<HeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MinWidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MinHeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MaxWidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MaxHeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<BoxSizingStyleValue>(), 16);
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
