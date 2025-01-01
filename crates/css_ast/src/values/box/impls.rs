pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

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
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<MarginTopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginRightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginBottomStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginLeftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<PaddingTopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingRightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingBottomStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingLeftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingStyleValue>(), 64);
		// assert_eq!(std::mem::size_of::<MarginTrimStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MarginLeftStyleValue, "auto");
		assert_parse!(MarginStyleValue, "1px 1px");
		assert_parse!(MarginStyleValue, "1px 2px");
		assert_parse!(MarginStyleValue, "1px 2px 3px 4px");
	}
}
