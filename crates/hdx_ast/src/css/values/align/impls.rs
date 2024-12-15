pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(AlignContentStyleValue, 1);
		// assert_size!(JustifyContentStyleValue, 1);
		// assert_size!(PlaceContentStyleValue, 1);
		// assert_size!(JustifySelfStyleValue, 1);
		// assert_size!(AlignSelfStyleValue, 1);
		// assert_size!(PlaceSelfStyleValue, 1);
		// assert_size!(JustifyItemsStyleValue, 1);
		// assert_size!(AlignItemsStyleValue, 1);
		// assert_size!(PlaceItemsStyleValue, 1);
		assert_size!(RowGapStyleValue, 16);
		assert_size!(ColumnGapStyleValue, 16);
		assert_size!(GapStyleValue, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(RowGapStyleValue, "normal");
		assert_parse!(ColumnGapStyleValue, "1px");
		assert_parse!(GapStyleValue, "normal 1px");
	}
}
