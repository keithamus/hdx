pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(AlignContent, 1);
		// assert_size!(JustifyContent, 1);
		// assert_size!(PlaceContent, 1);
		// assert_size!(JustifySelf, 1);
		// assert_size!(AlignSelf, 1);
		// assert_size!(PlaceSelf, 1);
		// assert_size!(JustifyItems, 1);
		// assert_size!(AlignItems, 1);
		// assert_size!(PlaceItems, 1);
		assert_size!(RowGap, 16);
		assert_size!(ColumnGap, 16);
		assert_size!(Gap, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(RowGap, "normal");
		assert_parse!(ColumnGap, "1px");
		assert_parse!(Gap, "normal 1px");
	}
}
