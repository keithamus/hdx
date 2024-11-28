pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BlockSize, 36);
		assert_size!(InlineSize, 36);
		assert_size!(MinBlockSize, 36);
		assert_size!(MinInlineSize, 36);
		assert_size!(MaxBlockSize, 36);
		assert_size!(MaxInlineSize, 36);
		assert_size!(MarginBlockStart, 16);
		assert_size!(MarginBlockEnd, 16);
		assert_size!(MarginInlineStart, 16);
		assert_size!(MarginInlineEnd, 16);
		assert_size!(MarginBlock, 32);
		assert_size!(MarginInline, 32);
		assert_size!(PaddingBlockStart, 12);
		assert_size!(PaddingBlockEnd, 12);
		assert_size!(PaddingInlineStart, 12);
		assert_size!(PaddingInlineEnd, 12);
		assert_size!(PaddingBlock, 24);
		assert_size!(PaddingInline, 24);
	}
}
