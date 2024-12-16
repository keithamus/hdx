pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(PositionStyleValue, 16);
		assert_size!(TopStyleValue, 16);
		assert_size!(RightStyleValue, 16);
		assert_size!(BottomStyleValue, 16);
		assert_size!(LeftStyleValue, 16);
		assert_size!(InsetBlockStartStyleValue, 16);
		assert_size!(InsetInlineStartStyleValue, 16);
		assert_size!(InsetBlockEndStyleValue, 16);
		assert_size!(InsetInlineEndStyleValue, 16);
		assert_size!(InsetBlockStyleValue, 32);
		assert_size!(InsetInlineStyleValue, 32);
		assert_size!(InsetStyleValue, 64);
		assert_size!(OverlayStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PositionStyleValue, "sticky");
		assert_parse!(InsetBlockStartStyleValue, "auto");
		assert_parse!(InsetStyleValue, "1px 2px");
		assert_parse!(InsetStyleValue, "1px 2px 3px 4px");
	}
}
