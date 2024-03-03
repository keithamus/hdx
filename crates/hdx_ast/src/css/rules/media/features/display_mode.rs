use crate::macros::discrete_media_feature;

discrete_media_feature!(DisplayModeMediaFeature[atom!("display-mode")] {
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
		assert_size!(DisplayModeMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DisplayModeMediaFeature, "display-mode");
		assert_parse!(DisplayModeMediaFeature, "display-mode: fullscreen");
		assert_parse!(DisplayModeMediaFeature, "display-mode: minimal-ui");
	}

	#[test]
	fn test_minify() {
		assert_minify!(DisplayModeMediaFeature, "display-mode: fullscreen", "display-mode:fullscreen");
		assert_minify!(DisplayModeMediaFeature, "display-mode: standalone", "display-mode:standalone");
		assert_minify!(DisplayModeMediaFeature, "display-mode: minimal-ui", "display-mode:minimal-ui");
		assert_minify!(DisplayModeMediaFeature, "display-mode: browser", "display-mode:browser");
		assert_minify!(DisplayModeMediaFeature, "display-mode: picture-in-picture", "display-mode:picture-in-picture");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DisplayModeMediaFeature, "display-mode:");
		assert_parse_error!(DisplayModeMediaFeature, "display-mode: pointer");
		assert_parse_error!(DisplayModeMediaFeature, "pointer: standalone");
	}
}
