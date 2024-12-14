pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(AnchorNameStyleValue, 32);
		assert_size!(AnchorScopeStyleValue, 40);
		assert_size!(PositionAnchorStyleValue, 16);
		assert_size!(PositionAreaStyleValue, 36);
		// assert_size!(PositionVisibilityStyleValue, 1);
		// assert_size!(PositionTryFallbacksStyleValue, 1);
		assert_size!(PositionTryOrderStyleValue, 16);
		// assert_size!(PositionTryStyleValue, 1);
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
