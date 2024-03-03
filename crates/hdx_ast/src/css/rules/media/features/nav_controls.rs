#[cfg(feature = "serde")]
use crate::macros::discrete_media_feature;

discrete_media_feature!(NavControlsMediaFeature[atom!("nav-controls")] {
	None: atom!("none"),
	Back: atom!("back"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(NavControlsMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NavControlsMediaFeature, "nav-controls");
		assert_parse!(NavControlsMediaFeature, "nav-controls: back");
		assert_parse!(NavControlsMediaFeature, "nav-controls: none");
	}

	#[test]
	fn test_minify() {
		assert_minify!(NavControlsMediaFeature, "nav-controls: back", "nav-controls:back");
		assert_minify!(NavControlsMediaFeature, "nav-controls: none", "nav-controls:none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(NavControlsMediaFeature, "nav-controls:");
		assert_parse_error!(NavControlsMediaFeature, "nav-controls: hoover");
	}
}
