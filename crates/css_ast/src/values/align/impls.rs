pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<AlignContentStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<JustifyContentStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PlaceContentStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<JustifySelfStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<AlignSelfStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PlaceSelfStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<JustifyItemsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<AlignItemsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PlaceItemsStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RowGapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnGapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<GapStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(RowGapStyleValue, "normal");
		assert_parse!(ColumnGapStyleValue, "1px");
		assert_parse!(GapStyleValue, "normal 1px");
	}
}
