pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BackgroundColor, 140);
		// assert_size!(BackgroundImage, 1);
		assert_size!(BackgroundRepeat, 32);
		assert_size!(BackgroundAttachment, 32);
		// assert_size!(BackgroundPosition, 1);
		assert_size!(BackgroundClip, 32);
		assert_size!(BackgroundOrigin, 32);
		// assert_size!(BackgroundSize, 1);
		// assert_size!(Background, 1);
		assert_size!(BorderImageSource, 184);
		// assert_size!(BorderImageSlice, 1);
		// assert_size!(BorderImageWidth, 1);
		// assert_size!(BorderImageOutset, 1);
		// assert_size!(BorderImageRepeat, 1);
		// assert_size!(BorderImage, 1);
		assert_size!(BackgroundRepeatX, 32);
		assert_size!(BackgroundRepeatY, 32);
		assert_size!(BackgroundRepeatBlock, 32);
		assert_size!(BackgroundRepeatInline, 32);
		// assert_size!(BackgroundPositionX, 1);
		// assert_size!(BackgroundPositionY, 1);
		// assert_size!(BackgroundPositionInline, 1);
		// assert_size!(BackgroundPositionBlock, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BackgroundRepeat, "repeat-x");
		assert_parse!(BackgroundRepeat, "space round");
	}
}
