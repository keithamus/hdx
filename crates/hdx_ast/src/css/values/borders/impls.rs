pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(BorderTopColor, 56);
		assert_size!(BorderRightColor, 56);
		assert_size!(BorderBottomColor, 56);
		assert_size!(BorderLeftColor, 56);
		assert_size!(BorderBlockStartColor, 56);
		assert_size!(BorderBlockEndColor, 56);
		assert_size!(BorderInlineStartColor, 56);
		assert_size!(BorderInlineEndColor, 56);
		// assert_size!(BorderColor, 1);
		assert_size!(BorderBlockColor, 112);
		assert_size!(BorderInlineColor, 112);
		assert_size!(BorderTopStyle, 1);
		assert_size!(BorderRightStyle, 1);
		assert_size!(BorderBottomStyle, 1);
		assert_size!(BorderLeftStyle, 1);
		assert_size!(BorderBlockStartStyle, 1);
		assert_size!(BorderBlockEndStyle, 1);
		assert_size!(BorderInlineEndStyle, 1);
		assert_size!(BorderBlockStyle, 2);
		assert_size!(BorderInlineStyle, 2);
		assert_size!(BorderTopWidth, 8);
		assert_size!(BorderRightWidth, 8);
		assert_size!(BorderBottomWidth, 8);
		assert_size!(BorderLeftWidth, 8);
		assert_size!(BorderBlockStartWidth, 8);
		assert_size!(BorderBlockEndWidth, 8);
		assert_size!(BorderInlineStartWidth, 8);
		assert_size!(BorderInlineEndWidth, 8);
		assert_size!(BorderBlockWidth, 16);
		assert_size!(BorderInlineWidth, 16);
		assert_size!(BorderTop, 48);
		assert_size!(BorderRight, 48);
		assert_size!(BorderBottom, 48);
		assert_size!(BorderLeft, 48);
		assert_size!(BorderBlockStart, 48);
		assert_size!(BorderBlockEnd, 48);
		assert_size!(BorderInlineStart, 48);
		assert_size!(BorderInlineEnd, 48);
		assert_size!(BorderBlock, 48);
		assert_size!(BorderInline, 48);
		assert_size!(BorderTopLeftRadius, 16);
		assert_size!(BorderTopRightRadius, 16);
		assert_size!(BorderBottomRightRadius, 16);
		assert_size!(BorderBottomLeftRadius, 16);
		assert_size!(BorderStartStartRadius, 16);
		assert_size!(BorderStartEndRadius, 16);
		assert_size!(BorderEndStartRadius, 16);
		assert_size!(BorderEndEndRadius, 16);
		// assert_size!(BorderTopRadius, 1);
		// assert_size!(BorderRightRadius, 1);
		// assert_size!(BorderBottomRadius, 1);
		// assert_size!(BorderLeftRadius, 1);
		// assert_size!(BorderBlockStartRadius, 1);
		// assert_size!(BorderBlockEndRadius, 1);
		// assert_size!(BorderInlineStartRadius, 1);
		// assert_size!(BorderInlineEndRadius, 1);
		// assert_size!(BorderRadius, 1);
		// assert_size!(CornerShape, 1);
		// assert_size!(Corners, 1);
		// assert_size!(BorderLimit, 1);
		// assert_size!(BorderClip, 1);
		// assert_size!(BorderClipTop, 1);
		// assert_size!(BorderClipRight, 1);
		// assert_size!(BorderClipBottom, 1);
		// assert_size!(BorderClipLeft, 1);
		// assert_size!(BoxShadowColor, 1);
		// assert_size!(BoxShadowOffset, 1);
		assert_size!(BoxShadowBlur, 24);
		assert_size!(BoxShadowSpread, 24);
		// assert_size!(BoxShadowPosition, 1);
		// assert_size!(BoxShadow, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderTopColor, "red");
	}
}
