pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlexDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexFlowStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<FlexStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FlexGrowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexShrinkStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexBasisStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<JustifyContentStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<AlignItemsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<AlignSelfStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<AlignContentStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FlexBasisStyleValue, "auto");
		assert_parse!(FlexBasisStyleValue, "4px");
	}
}
