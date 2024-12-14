pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FlexDirectionStyleValue, 16);
		assert_size!(FlexWrapStyleValue, 16);
		assert_size!(FlexFlowStyleValue, 32);
		// assert_size!(FlexStyleValue, 1);
		assert_size!(FlexGrowStyleValue, 12);
		assert_size!(FlexShrinkStyleValue, 12);
		assert_size!(FlexBasisStyleValue, 44);
		assert_size!(JustifyContentStyleValue, 16);
		assert_size!(AlignItemsStyleValue, 16);
		assert_size!(AlignSelfStyleValue, 16);
		assert_size!(AlignContentStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FlexBasisStyleValue, "auto");
		assert_parse!(FlexBasisStyleValue, "4px");
	}
}
