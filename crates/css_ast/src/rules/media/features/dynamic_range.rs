use css_parse::{discrete_feature, keyword_set};

discrete_feature!(DynamicRangeMediaFeature, "dynamic-range", DynamicRangeMediaFeatureKeyword);

keyword_set!(DynamicRangeMediaFeatureKeyword { Standard: "standard", High: "high" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range)");
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range:standard)");
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DynamicRangeMediaFeature, "(dynamic-range:)");
		assert_parse_error!(DynamicRangeMediaFeature, "(dynamic-range: pointer)");
		assert_parse_error!(DynamicRangeMediaFeature, "(pointer: standard)");
	}
}
