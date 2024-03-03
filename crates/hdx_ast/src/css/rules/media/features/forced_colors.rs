use crate::macros::discrete_media_feature;

discrete_media_feature!(ForcedColorsMediaFeature[atom!("forced-colors")] {
	None: atom!("none"),
	Active: atom!("active"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ForcedColorsMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ForcedColorsMediaFeature, "forced-colors");
		assert_parse!(ForcedColorsMediaFeature, "forced-colors: none");
		assert_parse!(ForcedColorsMediaFeature, "forced-colors: active");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ForcedColorsMediaFeature, "forced-colors: none", "forced-colors:none");
		assert_minify!(ForcedColorsMediaFeature, "forced-colors: active", "forced-colors:active");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ForcedColorsMediaFeature, "forced-colors:");
		assert_parse_error!(ForcedColorsMediaFeature, "forced-colors: pointer");
		assert_parse_error!(ForcedColorsMediaFeature, "pointer: none");
	}
}
