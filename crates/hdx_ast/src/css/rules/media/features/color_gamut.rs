use hdx_parser::discrete_feature;

discrete_feature!(ColorGamutMediaFeature[atom!("color-gamut")] {
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
		assert_size!(ColorGamutMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorGamutMediaFeature, "color-gamut");
		assert_parse!(ColorGamutMediaFeature, "color-gamut:srgb");
		assert_parse!(ColorGamutMediaFeature, "color-gamut:p3");
		assert_parse!(ColorGamutMediaFeature, "color-gamut:rec2020");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorGamutMediaFeature, "color-gamut:");
		assert_parse_error!(ColorGamutMediaFeature, "color-gamut: pointer");
	}
}
