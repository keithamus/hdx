pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(AnchorName, 32);
		assert_size!(AnchorScope, 40);
		assert_size!(PositionAnchor, 40);
		assert_size!(PositionArea, 36);
		// assert_size!(PositionVisibility, 1);
		// assert_size!(PositionTryFallbacks, 1);
		assert_size!(PositionTryOrder, 16);
		// assert_size!(PositionTry, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnchorName, "none");
		assert_parse!(AnchorName, "--foo,--bar");
		assert_parse!(AnchorScope, "all");
		assert_parse!(AnchorScope, "--foo,--bar");
		assert_parse!(PositionTryOrder, "normal");
	}
}
