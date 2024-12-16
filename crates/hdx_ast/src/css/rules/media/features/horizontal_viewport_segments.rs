use crate::css::units::CSSInt;
use hdx_parser::ranged_feature;

ranged_feature!(HorizontalViewportSegmentsMediaFeature[atom!("horizontal-viewport-segments")], CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(HorizontalViewportSegmentsMediaFeature, 92);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments:2");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments:8");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "min-horizontal-viewport-segments:2");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "max-horizontal-viewport-segments:2");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments<=3");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments>=5");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments>=8");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments=16");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "6=horizontal-viewport-segments");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "2<=horizontal-viewport-segments");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "2<horizontal-viewport-segments<4");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "4>horizontal-viewport-segments<8");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "4>=horizontal-viewport-segments<=8");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "4<=horizontal-viewport-segments>8");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments:");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments: > 10px");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "max-horizontal-viewport-segments > 10px");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "min-horizontal-viewport-segments > 10px");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "horizontal-viewport-segments: 1px");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "pointer: 1");
	}
}
