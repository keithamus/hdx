pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BackgroundColorStyleValue, 160);
		// assert_size!(BackgroundImageStyleValue, 1);
		assert_size!(BackgroundRepeatStyleValue, 32);
		assert_size!(BackgroundAttachmentStyleValue, 32);
		// assert_size!(BackgroundPositionStyleValue, 1);
		assert_size!(BackgroundClipStyleValue, 32);
		assert_size!(BackgroundOriginStyleValue, 32);
		// assert_size!(BackgroundSizeStyleValue, 1);
		// assert_size!(BackgroundStyleValue, 1);
		assert_size!(BorderImageSourceStyleValue, 208);
		// assert_size!(BorderImageSliceStyleValue, 1);
		// assert_size!(BorderImageWidthStyleValue, 1);
		// assert_size!(BorderImageOutsetStyleValue, 1);
		// assert_size!(BorderImageRepeatStyleValue, 1);
		// assert_size!(BorderImageStyleValue, 1);
		assert_size!(BackgroundRepeatXStyleValue, 32);
		assert_size!(BackgroundRepeatYStyleValue, 32);
		assert_size!(BackgroundRepeatBlockStyleValue, 32);
		assert_size!(BackgroundRepeatInlineStyleValue, 32);
		// assert_size!(BackgroundPositionXStyleValue, 1);
		// assert_size!(BackgroundPositionYStyleValue, 1);
		// assert_size!(BackgroundPositionInlineStyleValue, 1);
		// assert_size!(BackgroundPositionBlockStyleValue, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BackgroundRepeatStyleValue, "repeat-x");
		assert_parse!(BackgroundRepeatStyleValue, "space round");
	}
}
