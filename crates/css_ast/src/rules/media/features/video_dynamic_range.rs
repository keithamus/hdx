use css_parse::{discrete_feature, keyword_set};

discrete_feature!(VideoDynamicRangeMediaFeature, "video-dynamic-range", VideoDynamicRangeMediaFeatureKeyword);

keyword_set!(VideoDynamicRangeMediaFeatureKeyword { Standard: "standard", Hight: "high" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VideoDynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range)");
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:standard)");
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:)");
		assert_parse_error!(VideoDynamicRangeMediaFeature, "(video-dynamic-range: low)");
	}
}
