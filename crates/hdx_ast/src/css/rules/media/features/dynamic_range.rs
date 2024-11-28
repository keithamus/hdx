use hdx_parser::discrete_media_feature;

discrete_media_feature!(DynamicRangeMediaFeature[atom!("dynamic-range")] {
	Standard: atom!("standard"),
	High: atom!("high"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DynamicRangeMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeMediaFeature, "dynamic-range");
		assert_parse!(DynamicRangeMediaFeature, "dynamic-range:standard");
		assert_parse!(DynamicRangeMediaFeature, "dynamic-range:high");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DynamicRangeMediaFeature, "dynamic-range:");
		assert_parse_error!(DynamicRangeMediaFeature, "dynamic-range: pointer");
		assert_parse_error!(DynamicRangeMediaFeature, "pointer: standard");
	}
}
