pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FloatReferenceStyleValue, 16);
		assert_size!(FloatStyleValue, 68);
		assert_size!(ClearStyleValue, 16);
		assert_size!(FloatDeferStyleValue, 16);
		assert_size!(FloatOffsetStyleValue, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FloatStyleValue, "left");
		assert_parse!(FloatStyleValue, "snap-block(1px,near)");
		assert_parse!(FloatStyleValue, "snap-inline(1px,near)");
	}
}
