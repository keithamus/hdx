use hdx_parser::discrete_media_feature;

discrete_media_feature!(HoverMediaFeature[atom!("hover")] {
	None: atom!("none"),
	Hover: atom!("hover"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(HoverMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HoverMediaFeature, "hover");
		assert_parse!(HoverMediaFeature, "hover:hover");
		assert_parse!(HoverMediaFeature, "hover:none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HoverMediaFeature, "hover:");
		assert_parse_error!(HoverMediaFeature, "hover: hoover");
	}
}
