use hdx_parser::discrete_media_feature;

discrete_media_feature!(AnyHoverMediaFeature[atom!("any-hover")] {
	None: atom!("none"),
	Hover: atom!("hover"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AnyHoverMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnyHoverMediaFeature, "any-hover");
		assert_parse!(AnyHoverMediaFeature, "any-hover:hover");
		assert_parse!(AnyHoverMediaFeature, "any-hover:none");
	}
}
