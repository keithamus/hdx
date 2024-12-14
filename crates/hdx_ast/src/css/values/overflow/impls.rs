pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OverflowXStyleValue, 16);
		assert_size!(OverflowYStyleValue, 16);
		assert_size!(OverflowBlockStyleValue, 16);
		assert_size!(OverflowInlineStyleValue, 16);
		assert_size!(OverflowStyleValue, 32);
		// assert_size!(OverflowClipMarginStyleValue, 12);
		assert_size!(ScrollBehaviorStyleValue, 16);
		// assert_size!(ScrollbarGutterStyleValue, 12);
		// assert_size!(TextOverflowStyleValue, 12);
		assert_size!(OverflowClipMarginTopStyleValue, 32);
		assert_size!(OverflowClipMarginRightStyleValue, 32);
		assert_size!(OverflowClipMarginBottomStyleValue, 32);
		assert_size!(OverflowClipMarginLeftStyleValue, 32);
		assert_size!(OverflowClipMarginInlineStartStyleValue, 32);
		assert_size!(OverflowClipMarginBlockEndStyleValue, 32);
		assert_size!(OverflowClipMarginInlineEndStyleValue, 32);
		assert_size!(OverflowClipMarginInlineStyleValue, 32);
		assert_size!(OverflowClipMarginBlockStyleValue, 32);
		assert_size!(BlockEllipsisStyleValue, 16);
		// assert_size!(LineClampStyleValue, 12);
		assert_size!(WebkitLineClampStyleValue, 16);
		assert_size!(MaxLinesStyleValue, 16);
		assert_size!(ContinueStyleValue, 16);
		assert_size!(ScrollMarkerGroupStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowXStyleValue, "scroll");
		assert_parse!(OverflowStyleValue, "hidden scroll");
	}
}
