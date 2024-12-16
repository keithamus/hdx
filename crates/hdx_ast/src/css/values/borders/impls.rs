pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(BorderTopColorStyleValue, 160);
		assert_size!(BorderRightColorStyleValue, 160);
		assert_size!(BorderBottomColorStyleValue, 160);
		assert_size!(BorderLeftColorStyleValue, 160);
		assert_size!(BorderBlockStartColorStyleValue, 160);
		assert_size!(BorderBlockEndColorStyleValue, 160);
		assert_size!(BorderInlineStartColorStyleValue, 160);
		assert_size!(BorderInlineEndColorStyleValue, 160);
		// assert_size!(BorderColorStyleValue, 1);
		assert_size!(BorderBlockColorStyleValue, 320);
		assert_size!(BorderInlineColorStyleValue, 320);
		assert_size!(BorderTopStyleStyleValue, 16);
		assert_size!(BorderRightStyleStyleValue, 16);
		assert_size!(BorderBottomStyleStyleValue, 16);
		assert_size!(BorderLeftStyleStyleValue, 16);
		assert_size!(BorderBlockStartStyleStyleValue, 16);
		assert_size!(BorderBlockEndStyleStyleValue, 16);
		assert_size!(BorderInlineEndStyleStyleValue, 16);
		assert_size!(BorderBlockStyleStyleValue, 32);
		assert_size!(BorderInlineStyleStyleValue, 32);
		assert_size!(BorderTopWidthStyleValue, 16);
		assert_size!(BorderRightWidthStyleValue, 16);
		assert_size!(BorderBottomWidthStyleValue, 16);
		assert_size!(BorderLeftWidthStyleValue, 16);
		assert_size!(BorderBlockStartWidthStyleValue, 16);
		assert_size!(BorderBlockEndWidthStyleValue, 16);
		assert_size!(BorderInlineStartWidthStyleValue, 16);
		assert_size!(BorderInlineEndWidthStyleValue, 16);
		assert_size!(BorderBlockWidthStyleValue, 32);
		assert_size!(BorderInlineWidthStyleValue, 32);
		assert_size!(BorderTopStyleValue, 192);
		assert_size!(BorderRightStyleValue, 192);
		assert_size!(BorderBottomStyleValue, 192);
		assert_size!(BorderLeftStyleValue, 192);
		assert_size!(BorderBlockStartStyleValue, 192);
		assert_size!(BorderBlockEndStyleValue, 192);
		assert_size!(BorderInlineStartStyleValue, 192);
		assert_size!(BorderInlineEndStyleValue, 192);
		assert_size!(BorderBlockStyleValue, 192);
		assert_size!(BorderInlineStyleValue, 192);
		assert_size!(BorderTopLeftRadiusStyleValue, 32);
		assert_size!(BorderTopRightRadiusStyleValue, 32);
		assert_size!(BorderBottomRightRadiusStyleValue, 32);
		assert_size!(BorderBottomLeftRadiusStyleValue, 32);
		assert_size!(BorderStartStartRadiusStyleValue, 32);
		assert_size!(BorderStartEndRadiusStyleValue, 32);
		assert_size!(BorderEndStartRadiusStyleValue, 32);
		assert_size!(BorderEndEndRadiusStyleValue, 32);
		// assert_size!(BorderTopRadiusStyleValue, 1);
		// assert_size!(BorderRightRadiusStyleValue, 1);
		// assert_size!(BorderBottomRadiusStyleValue, 1);
		// assert_size!(BorderLeftRadiusStyleValue, 1);
		// assert_size!(BorderBlockStartRadiusStyleValue, 1);
		// assert_size!(BorderBlockEndRadiusStyleValue, 1);
		// assert_size!(BorderInlineStartRadiusStyleValue, 1);
		// assert_size!(BorderInlineEndRadiusStyleValue, 1);
		// assert_size!(BorderRadiusStyleValue, 1);
		// assert_size!(CornerShapeStyleValue, 1);
		// assert_size!(CornersStyleValue, 1);
		// assert_size!(BorderLimitStyleValue, 1);
		// assert_size!(BorderClipStyleValue, 1);
		// assert_size!(BorderClipTopStyleValue, 1);
		// assert_size!(BorderClipRightStyleValue, 1);
		// assert_size!(BorderClipBottomStyleValue, 1);
		// assert_size!(BorderClipLeftStyleValue, 1);
		// assert_size!(BoxShadowColorStyleValue, 1);
		// assert_size!(BoxShadowOffsetStyleValue, 1);
		assert_size!(BoxShadowBlurStyleValue, 32);
		assert_size!(BoxShadowSpreadStyleValue, 32);
		// assert_size!(BoxShadowPositionStyleValue, 1);
		// assert_size!(BoxShadowStyleValue, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderTopColorStyleValue, "red");
	}
}
