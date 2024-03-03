use crate::macros::discrete_media_feature;

discrete_media_feature!(VideoColorGamutMediaFeature[atom!("video-color-gamut")] {
	Srgb: atom!("srgb"),
	P3: atom!("p3"),
	Rec2020: atom!("rec2020"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(VideoColorGamutMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VideoColorGamutMediaFeature, "video-color-gamut");
		assert_parse!(VideoColorGamutMediaFeature, "video-color-gamut: srgb");
		assert_parse!(VideoColorGamutMediaFeature, "video-color-gamut: p3");
		assert_parse!(VideoColorGamutMediaFeature, "video-color-gamut: rec2020");
	}

	#[test]
	fn test_minify() {
		assert_minify!(VideoColorGamutMediaFeature, "video-color-gamut: srgb", "video-color-gamut:srgb");
		assert_minify!(VideoColorGamutMediaFeature, "video-color-gamut: p3", "video-color-gamut:p3");
		assert_minify!(VideoColorGamutMediaFeature, "video-color-gamut: rec2020", "video-color-gamut:rec2020");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VideoColorGamutMediaFeature, "video-color-gamut:");
		assert_parse_error!(VideoColorGamutMediaFeature, "video-color-gamut: rec");
	}
}
