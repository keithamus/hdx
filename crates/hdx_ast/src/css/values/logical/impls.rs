pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BlockSize, 12);
		assert_size!(InlineSize, 12);
		assert_size!(MinBlockSize, 12);
		assert_size!(MinInlineSize, 12);
		assert_size!(MaxBlockSize, 12);
		assert_size!(MaxInlineSize, 12);
		assert_size!(MarginBlockStart, 8);
		assert_size!(MarginBlockEnd, 8);
		assert_size!(MarginInlineStart, 8);
		assert_size!(MarginInlineEnd, 8);
		assert_size!(MarginBlock, 16);
		assert_size!(MarginInline, 16);
		assert_size!(PaddingBlockStart, 8);
		assert_size!(PaddingBlockEnd, 8);
		assert_size!(PaddingInlineStart, 8);
		assert_size!(PaddingInlineEnd, 8);
		assert_size!(PaddingBlock, 16);
		assert_size!(PaddingInline, 16);
	}
}
