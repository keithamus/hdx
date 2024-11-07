pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OverflowX, 1);
		assert_size!(OverflowY, 1);
		assert_size!(OverflowBlock, 1);
		assert_size!(OverflowInline, 1);
		assert_size!(Overflow, 2);
		// assert_size!(OverflowClipMargin, 12);
		assert_size!(ScrollBehavior, 1);
		// assert_size!(ScrollbarGutter, 12);
		// assert_size!(TextOverflow, 12);
		assert_size!(OverflowClipMarginTop, 12);
		assert_size!(OverflowClipMarginRight, 12);
		assert_size!(OverflowClipMarginBottom, 12);
		assert_size!(OverflowClipMarginLeft, 12);
		assert_size!(OverflowClipMarginInlineStart, 12);
		assert_size!(OverflowClipMarginBlockEnd, 12);
		assert_size!(OverflowClipMarginInlineEnd, 12);
		assert_size!(OverflowClipMarginInline, 12);
		assert_size!(OverflowClipMarginBlock, 12);
		assert_size!(BlockEllipsis, 24);
		// assert_size!(LineClamp, 12);
		assert_size!(WebkitLineClamp, 8);
		assert_size!(MaxLines, 8);
		assert_size!(Continue, 1);
		assert_size!(ScrollMarkerGroup, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowX, "scroll");
		assert_parse!(Overflow, "hidden scroll");
	}
}
