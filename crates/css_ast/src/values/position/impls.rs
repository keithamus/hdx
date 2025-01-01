pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<PositionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BottomStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LeftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InsetBlockStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InsetInlineStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InsetBlockEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InsetInlineEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InsetBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<InsetInlineStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<InsetStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<OverlayStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PositionStyleValue, "sticky");
		assert_parse!(InsetBlockStartStyleValue, "auto");
		assert_parse!(InsetStyleValue, "1px 2px");
		assert_parse!(InsetStyleValue, "1px 2px 3px 4px");
	}
}
