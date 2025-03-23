pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<AnchorNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnchorScopeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<PositionAnchorStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PositionAreaStyleValue>(), 36);
		// assert_eq!(std::mem::size_of::<PositionVisibilityStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PositionTryFallbacksStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<PositionTryOrderStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<PositionTryStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnchorNameStyleValue, "none");
		assert_parse!(AnchorNameStyleValue, "--foo,--bar");
		assert_parse!(AnchorScopeStyleValue, "all");
		assert_parse!(AnchorScopeStyleValue, "--foo,--bar");
		assert_parse!(PositionTryOrderStyleValue, "normal");
	}
}
