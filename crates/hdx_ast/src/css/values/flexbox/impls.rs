pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FlexDirection, 16);
		assert_size!(FlexWrap, 16);
		assert_size!(FlexFlow, 32);
		// assert_size!(Flex, 1);
		assert_size!(FlexGrow, 8);
		assert_size!(FlexShrink, 8);
		assert_size!(FlexBasis, 36);
		assert_size!(JustifyContent, 16);
		assert_size!(AlignItems, 16);
		assert_size!(AlignSelf, 16);
		assert_size!(AlignContent, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FlexBasis, "auto");
		assert_parse!(FlexBasis, "4px");
	}
}
