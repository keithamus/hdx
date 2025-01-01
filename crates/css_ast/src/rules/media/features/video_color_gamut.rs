use css_parse::{discrete_feature, keyword_set};

discrete_feature!(VideoColorGamutMediaFeature, "video-color-gamut", VideoColorGamutMediaFeatureKeyword);

keyword_set!(VideoColorGamutMediaFeatureKeyword { Srgb: "srgb", P3: "p3", Rec2020: "rec2020" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VideoColorGamutMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VideoColorGamutMediaFeature, "(video-color-gamut)");
		assert_parse!(VideoColorGamutMediaFeature, "(video-color-gamut:srgb)");
		assert_parse!(VideoColorGamutMediaFeature, "(video-color-gamut:p3)");
		assert_parse!(VideoColorGamutMediaFeature, "(video-color-gamut:rec2020)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VideoColorGamutMediaFeature, "(video-color-gamut:)");
		assert_parse_error!(VideoColorGamutMediaFeature, "(video-color-gamut: rec)");
	}
}
