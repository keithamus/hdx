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
		assert_size!(ScrollSnapStop, 16);
		assert_size!(ScrollPaddingTop, 16);
		assert_size!(ScrollPaddingRight, 16);
		assert_size!(ScrollPaddingBottom, 16);
		assert_size!(ScrollPaddingLeft, 16);
		assert_size!(ScrollPaddingInlineStart, 16);
		assert_size!(ScrollPaddingBlockStart, 16);
		assert_size!(ScrollPaddingInlineEnd, 16);
		assert_size!(ScrollPaddingBlockEnd, 16);
		// assert_size!(ScrollPaddingBlock, 2);
		// assert_size!(ScrollPaddingInline, 2);
		assert_size!(ScrollMarginTop, 12);
		assert_size!(ScrollMarginRight, 12);
		assert_size!(ScrollMarginBottom, 12);
		assert_size!(ScrollMarginLeft, 12);
		assert_size!(ScrollMarginBlockStart, 12);
		assert_size!(ScrollMarginInlineStart, 12);
		assert_size!(ScrollMarginBlockEnd, 12);
		assert_size!(ScrollMarginInlineEnd, 12);
		assert_size!(ScrollMarginBlock, 24);
		assert_size!(ScrollMarginInline, 24);
		assert_size!(ScrollStartTarget, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScrollPaddingTop, "1px");
		assert_parse!(ScrollMarginTop, "1px");
	}
}
