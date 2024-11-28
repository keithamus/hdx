use hdx_parser::discrete_media_feature;

discrete_media_feature!(OrientationMediaFeature[atom!("orientation")] {
	Portrait: atom!("portrait"),
	Landscape: atom!("landscape"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OrientationMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OrientationMediaFeature, "orientation");
		assert_parse!(OrientationMediaFeature, "orientation:portrait");
		assert_parse!(OrientationMediaFeature, "orientation:landscape");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OrientationMediaFeature, "orientation:");
		assert_parse_error!(OrientationMediaFeature, "orientation: landscope");
	}
}
