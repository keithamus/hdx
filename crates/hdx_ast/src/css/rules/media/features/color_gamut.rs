use crate::macros::discrete_media_feature;

discrete_media_feature!(ColorGamutMediaFeature[atom!("color-gamut")] {
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
		assert_size!(ColorGamutMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorGamutMediaFeature, "color-gamut");
		assert_parse!(ColorGamutMediaFeature, "color-gamut: srgb");
		assert_parse!(ColorGamutMediaFeature, "color-gamut: p3");
		assert_parse!(ColorGamutMediaFeature, "color-gamut: rec2020");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ColorGamutMediaFeature, "color-gamut: srgb", "color-gamut:srgb");
		assert_minify!(ColorGamutMediaFeature, "color-gamut: p3", "color-gamut:p3");
		assert_minify!(ColorGamutMediaFeature, "color-gamut: rec2020", "color-gamut:rec2020");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorGamutMediaFeature, "color-gamut:");
		assert_parse_error!(ColorGamutMediaFeature, "color-gamut: pointer");
	}
}
