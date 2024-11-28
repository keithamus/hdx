pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OverflowX, 16);
		assert_size!(OverflowY, 16);
		assert_size!(OverflowBlock, 16);
		assert_size!(OverflowInline, 16);
		assert_size!(Overflow, 32);
		// assert_size!(OverflowClipMargin, 12);
		assert_size!(ScrollBehavior, 16);
		// assert_size!(ScrollbarGutter, 12);
		// assert_size!(TextOverflow, 12);
		assert_size!(OverflowClipMarginTop, 28);
		assert_size!(OverflowClipMarginRight, 28);
		assert_size!(OverflowClipMarginBottom, 28);
		assert_size!(OverflowClipMarginLeft, 28);
		assert_size!(OverflowClipMarginInlineStart, 28);
		assert_size!(OverflowClipMarginBlockEnd, 28);
		assert_size!(OverflowClipMarginInlineEnd, 28);
		assert_size!(OverflowClipMarginInline, 28);
		assert_size!(OverflowClipMarginBlock, 28);
		assert_size!(BlockEllipsis, 16);
		// assert_size!(LineClamp, 12);
		assert_size!(WebkitLineClamp, 16);
		assert_size!(MaxLines, 16);
		assert_size!(Continue, 16);
		assert_size!(ScrollMarkerGroup, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowX, "scroll");
		assert_parse!(Overflow, "hidden scroll");
	}
}
