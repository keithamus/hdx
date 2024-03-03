use crate::macros::discrete_media_feature;

discrete_media_feature!(VideoDynamicRangeMediaFeature[atom!("video-dynamic-range")] {
	Standard: atom!("standard"),
	Hight: atom!("high"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(VideoDynamicRangeMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VideoDynamicRangeMediaFeature, "video-dynamic-range");
		assert_parse!(VideoDynamicRangeMediaFeature, "video-dynamic-range: standard");
		assert_parse!(VideoDynamicRangeMediaFeature, "video-dynamic-range: high");
	}

	#[test]
	fn test_minify() {
		assert_minify!(VideoDynamicRangeMediaFeature, "video-dynamic-range: standard", "video-dynamic-range:standard");
		assert_minify!(VideoDynamicRangeMediaFeature, "video-dynamic-range: high", "video-dynamic-range:high");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VideoDynamicRangeMediaFeature, "video-dynamic-range:");
		assert_parse_error!(VideoDynamicRangeMediaFeature, "video-dynamic-range: low");
	}
}
