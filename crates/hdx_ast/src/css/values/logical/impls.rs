pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BlockSizeStyleValue, 36);
		assert_size!(InlineSizeStyleValue, 36);
		assert_size!(MinBlockSizeStyleValue, 36);
		assert_size!(MinInlineSizeStyleValue, 36);
		assert_size!(MaxBlockSizeStyleValue, 36);
		assert_size!(MaxInlineSizeStyleValue, 36);
		assert_size!(MarginBlockStartStyleValue, 16);
		assert_size!(MarginBlockEndStyleValue, 16);
		assert_size!(MarginInlineStartStyleValue, 16);
		assert_size!(MarginInlineEndStyleValue, 16);
		assert_size!(MarginBlockStyleValue, 32);
		assert_size!(MarginInlineStyleValue, 32);
		assert_size!(PaddingBlockStartStyleValue, 12);
		assert_size!(PaddingBlockEndStyleValue, 12);
		assert_size!(PaddingInlineStartStyleValue, 12);
		assert_size!(PaddingInlineEndStyleValue, 12);
		assert_size!(PaddingBlockStyleValue, 24);
		assert_size!(PaddingInlineStyleValue, 24);
	}
}
