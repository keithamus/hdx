use crate::css::units::CSSInt;
use hdx_parser::ranged_media_feature;

ranged_media_feature!(VerticalViewportSegmentsMediaFeature[atom!("vertical-viewport-segments")], CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(VerticalViewportSegmentsMediaFeature, 68);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments:2");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments:8");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "min-vertical-viewport-segments:2");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "max-vertical-viewport-segments:2");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments<=3");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments>=5");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments>=8");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments=16");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "6=vertical-viewport-segments");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "2<=vertical-viewport-segments");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "2<vertical-viewport-segments<4");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "4>vertical-viewport-segments<8");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "4>=vertical-viewport-segments<=8");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "4<=vertical-viewport-segments>8");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments:");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments: > 10px");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "max-vertical-viewport-segments > 10px");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "min-vertical-viewport-segments > 10px");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "vertical-viewport-segments: 1px");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "pointer: 1");
	}
}
