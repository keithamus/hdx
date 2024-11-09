pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(ScrollSnapType, 2);
		// assert_size!(ScrollPadding, 2);
		// assert_size!(ScrollMargin, 2);
		// assert_size!(ScrollSnapAlign, 2);
		assert_size!(ScrollSnapStop, 1);
		assert_size!(ScrollPaddingTop, 8);
		assert_size!(ScrollPaddingRight, 8);
		assert_size!(ScrollPaddingBottom, 8);
		assert_size!(ScrollPaddingLeft, 8);
		assert_size!(ScrollPaddingInlineStart, 8);
		assert_size!(ScrollPaddingBlockStart, 8);
		assert_size!(ScrollPaddingInlineEnd, 8);
		assert_size!(ScrollPaddingBlockEnd, 8);
		// assert_size!(ScrollPaddingBlock, 2);
		// assert_size!(ScrollPaddingInline, 2);
		assert_size!(ScrollMarginTop, 8);
		assert_size!(ScrollMarginRight, 8);
		assert_size!(ScrollMarginBottom, 8);
		assert_size!(ScrollMarginLeft, 8);
		assert_size!(ScrollMarginBlockStart, 8);
		assert_size!(ScrollMarginInlineStart, 8);
		assert_size!(ScrollMarginBlockEnd, 8);
		assert_size!(ScrollMarginInlineEnd, 8);
		assert_size!(ScrollMarginBlock, 16);
		assert_size!(ScrollMarginInline, 16);
		assert_size!(ScrollStartTarget, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScrollPaddingTop, "1px");
		assert_parse!(ScrollMarginTop, "1px");
	}
}
