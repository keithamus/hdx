use crate::units::CSSInt;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(ColorIndexMediaFeatureKeyword {
	ColorIndex: "color-index",
	MaxColorIndex: "max-color-index",
	MinColorIndex: "min-color-index",
});

impl RangedFeatureKeyword for ColorIndexMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxColorIndex(_) | Self::MinColorIndex(_))
	}
}

ranged_feature!(ColorIndexMediaFeature, ColorIndexMediaFeatureKeyword, CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorIndexMediaFeature>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorIndexMediaFeature, "(color-index:2)");
		assert_parse!(ColorIndexMediaFeature, "(color-index:8)");
		assert_parse!(ColorIndexMediaFeature, "(min-color-index:2)");
		assert_parse!(ColorIndexMediaFeature, "(max-color-index:2)");
		assert_parse!(ColorIndexMediaFeature, "(color-index<=3)");
		assert_parse!(ColorIndexMediaFeature, "(color-index>=5)");
		assert_parse!(ColorIndexMediaFeature, "(color-index>=8)");
		assert_parse!(ColorIndexMediaFeature, "(color-index=16)");
		assert_parse!(ColorIndexMediaFeature, "(6=color-index)");
		assert_parse!(ColorIndexMediaFeature, "(2<=color-index)");
		assert_parse!(ColorIndexMediaFeature, "(2<color-index<4)");
		assert_parse!(ColorIndexMediaFeature, "(4>color-index<8)");
		assert_parse!(ColorIndexMediaFeature, "(4>=color-index<=8)");
		assert_parse!(ColorIndexMediaFeature, "(4<=color-index>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorIndexMediaFeature, "(color-index:)");
		assert_parse_error!(ColorIndexMediaFeature, "(color-index: > 10px)");
		assert_parse_error!(ColorIndexMediaFeature, "(max-color-index > 10px)");
		assert_parse_error!(ColorIndexMediaFeature, "(min-color-index > 10px)");
		assert_parse_error!(ColorIndexMediaFeature, "(color-index: 1px)");
		assert_parse_error!(ColorIndexMediaFeature, "(color-index: red)");
		assert_parse_error!(ColorIndexMediaFeature, "(pointer: 1)");
	}
}
