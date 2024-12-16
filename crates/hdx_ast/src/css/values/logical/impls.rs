pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BlockSizeStyleValue, 44);
		assert_size!(InlineSizeStyleValue, 44);
		assert_size!(MinBlockSizeStyleValue, 44);
		assert_size!(MinInlineSizeStyleValue, 44);
		assert_size!(MaxBlockSizeStyleValue, 44);
		assert_size!(MaxInlineSizeStyleValue, 44);
		assert_size!(MarginBlockStartStyleValue, 16);
		assert_size!(MarginBlockEndStyleValue, 16);
		assert_size!(MarginInlineStartStyleValue, 16);
		assert_size!(MarginInlineEndStyleValue, 16);
		assert_size!(MarginBlockStyleValue, 32);
		assert_size!(MarginInlineStyleValue, 32);
		assert_size!(PaddingBlockStartStyleValue, 16);
		assert_size!(PaddingBlockEndStyleValue, 16);
		assert_size!(PaddingInlineStartStyleValue, 16);
		assert_size!(PaddingInlineEndStyleValue, 16);
		assert_size!(PaddingBlockStyleValue, 32);
		assert_size!(PaddingInlineStyleValue, 32);
	}
}
