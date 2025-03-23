pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowXStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowYStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowBlockStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowInlineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<OverflowClipMarginStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<ScrollBehaviorStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<ScrollbarGutterStyleValue>(), 12);
		// assert_eq!(std::mem::size_of::<TextOverflowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<OverflowClipMarginTopStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginRightStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBottomStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginLeftStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBlockEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BlockEllipsisStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<LineClampStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<WebkitLineClampStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaxLinesStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ContinueStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarkerGroupStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowXStyleValue, "scroll");
		assert_parse!(OverflowStyleValue, "hidden scroll");
	}
}
