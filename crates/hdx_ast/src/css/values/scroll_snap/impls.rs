pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(ScrollSnapTypeStyleValue, 2);
		// assert_size!(ScrollPaddingStyleValue, 2);
		// assert_size!(ScrollMarginStyleValue, 2);
		// assert_size!(ScrollSnapAlignStyleValue, 2);
		assert_size!(ScrollSnapStopStyleValue, 16);
		assert_size!(ScrollPaddingTopStyleValue, 16);
		assert_size!(ScrollPaddingRightStyleValue, 16);
		assert_size!(ScrollPaddingBottomStyleValue, 16);
		assert_size!(ScrollPaddingLeftStyleValue, 16);
		assert_size!(ScrollPaddingInlineStartStyleValue, 16);
		assert_size!(ScrollPaddingBlockStartStyleValue, 16);
		assert_size!(ScrollPaddingInlineEndStyleValue, 16);
		assert_size!(ScrollPaddingBlockEndStyleValue, 16);
		// assert_size!(ScrollPaddingBlockStyleValue, 2);
		// assert_size!(ScrollPaddingInlineStyleValue, 2);
		assert_size!(ScrollMarginTopStyleValue, 12);
		assert_size!(ScrollMarginRightStyleValue, 12);
		assert_size!(ScrollMarginBottomStyleValue, 12);
		assert_size!(ScrollMarginLeftStyleValue, 12);
		assert_size!(ScrollMarginBlockStartStyleValue, 12);
		assert_size!(ScrollMarginInlineStartStyleValue, 12);
		assert_size!(ScrollMarginBlockEndStyleValue, 12);
		assert_size!(ScrollMarginInlineEndStyleValue, 12);
		assert_size!(ScrollMarginBlockStyleValue, 24);
		assert_size!(ScrollMarginInlineStyleValue, 24);
		assert_size!(ScrollStartTargetStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScrollPaddingTopStyleValue, "1px");
		assert_parse!(ScrollMarginTopStyleValue, "1px");
	}
}
