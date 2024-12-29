use crate::units::CSSInt;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(VerticalViewportSegmentsMediaFeatureKeyword {
	VerticalViewportSegments: "vertical-viewport-segments",
	MaxVerticalViewportSegments: "max-vertical-viewport-segments",
	MinVerticalViewportSegments: "min-vertical-viewport-segments",
});

impl RangedFeatureKeyword for VerticalViewportSegmentsMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxVerticalViewportSegments(_) | Self::MinVerticalViewportSegments(_))
	}
}

ranged_feature!(VerticalViewportSegmentsMediaFeature, VerticalViewportSegmentsMediaFeatureKeyword, CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VerticalViewportSegmentsMediaFeature>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:2)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:8)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(min-vertical-viewport-segments:2)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(max-vertical-viewport-segments:2)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments<=3)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments>=5)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments>=8)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments=16)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(6=vertical-viewport-segments)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(2<=vertical-viewport-segments)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(2<vertical-viewport-segments<4)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(4>vertical-viewport-segments<8)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(4>=vertical-viewport-segments<=8)");
		assert_parse!(VerticalViewportSegmentsMediaFeature, "(4<=vertical-viewport-segments>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:)");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments: > 10px)");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(max-vertical-viewport-segments > 10px)");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(min-vertical-viewport-segments > 10px)");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments: 1px)");
		assert_parse_error!(VerticalViewportSegmentsMediaFeature, "(pointer: 1)");
	}
}
