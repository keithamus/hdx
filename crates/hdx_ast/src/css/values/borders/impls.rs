pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(BorderTopColor, 144);
		assert_size!(BorderRightColor, 144);
		assert_size!(BorderBottomColor, 144);
		assert_size!(BorderLeftColor, 144);
		assert_size!(BorderBlockStartColor, 144);
		assert_size!(BorderBlockEndColor, 144);
		assert_size!(BorderInlineStartColor, 144);
		assert_size!(BorderInlineEndColor, 144);
		// assert_size!(BorderColor, 1);
		assert_size!(BorderBlockColor, 288);
		assert_size!(BorderInlineColor, 288);
		assert_size!(BorderTopStyle, 16);
		assert_size!(BorderRightStyle, 16);
		assert_size!(BorderBottomStyle, 16);
		assert_size!(BorderLeftStyle, 16);
		assert_size!(BorderBlockStartStyle, 16);
		assert_size!(BorderBlockEndStyle, 16);
		assert_size!(BorderInlineEndStyle, 16);
		assert_size!(BorderBlockStyle, 32);
		assert_size!(BorderInlineStyle, 32);
		assert_size!(BorderTopWidth, 16);
		assert_size!(BorderRightWidth, 16);
		assert_size!(BorderBottomWidth, 16);
		assert_size!(BorderLeftWidth, 16);
		assert_size!(BorderBlockStartWidth, 16);
		assert_size!(BorderBlockEndWidth, 16);
		assert_size!(BorderInlineStartWidth, 16);
		assert_size!(BorderInlineEndWidth, 16);
		assert_size!(BorderBlockWidth, 32);
		assert_size!(BorderInlineWidth, 32);
		assert_size!(BorderTop, 172);
		assert_size!(BorderRight, 172);
		assert_size!(BorderBottom, 172);
		assert_size!(BorderLeft, 172);
		assert_size!(BorderBlockStart, 172);
		assert_size!(BorderBlockEnd, 172);
		assert_size!(BorderInlineStart, 172);
		assert_size!(BorderInlineEnd, 172);
		assert_size!(BorderBlock, 172);
		assert_size!(BorderInline, 172);
		assert_size!(BorderTopLeftRadius, 24);
		assert_size!(BorderTopRightRadius, 24);
		assert_size!(BorderBottomRightRadius, 24);
		assert_size!(BorderBottomLeftRadius, 24);
		assert_size!(BorderStartStartRadius, 24);
		assert_size!(BorderStartEndRadius, 24);
		assert_size!(BorderEndStartRadius, 24);
		assert_size!(BorderEndEndRadius, 24);
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
		assert_size!(BoxShadowBlur, 32);
		assert_size!(BoxShadowSpread, 32);
		// assert_size!(BoxShadowPosition, 1);
		// assert_size!(BoxShadow, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderTopColor, "red");
	}
}
