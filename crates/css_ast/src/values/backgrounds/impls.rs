pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BackgroundColorStyleValue>(), 160);
		// assert_eq!(std::mem::size_of::<BackgroundImageStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundAttachmentStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundPositionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundOriginStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundSizeStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderImageSourceStyleValue>(), 208);
		// assert_eq!(std::mem::size_of::<BorderImageSliceStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageWidthStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageOutsetStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageRepeatStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundRepeatXStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatYStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatInlineStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundPositionXStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionYStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionInlineStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionBlockStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BackgroundRepeatStyleValue, "repeat-x");
		assert_parse!(BackgroundRepeatStyleValue, "space round");
	}
}
