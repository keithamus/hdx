use hdx_parser::discrete_feature;

discrete_feature!(DisplayModeMediaFeature[atom!("display-mode")] {
	Fullscreen: atom!("fullscreen"),
	Standalone: atom!("standalone"),
	MinimalUi: atom!("minimal-ui"),
	Browser: atom!("browser"),
	PictureInPicture: atom!("picture-in-picture"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DisplayModeMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DisplayModeMediaFeature, "display-mode");
		assert_parse!(DisplayModeMediaFeature, "display-mode:fullscreen");
		assert_parse!(DisplayModeMediaFeature, "display-mode:minimal-ui");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DisplayModeMediaFeature, "display-mode:");
		assert_parse_error!(DisplayModeMediaFeature, "display-mode: pointer");
		assert_parse_error!(DisplayModeMediaFeature, "pointer: standalone");
	}
}
