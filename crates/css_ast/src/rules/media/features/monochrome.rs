use crate::units::CSSInt;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(MonochromeMediaFeatureKeyword {
	Monochrome: "monochrome",
	MaxMonochrome: "max-monochrome",
	MinMonochrome: "min-monochrome",
});

impl RangedFeatureKeyword for MonochromeMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxMonochrome(_) | Self::MinMonochrome(_))
	}
}

ranged_feature!(MonochromeMediaFeature, MonochromeMediaFeatureKeyword, CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MonochromeMediaFeature>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MonochromeMediaFeature, "(monochrome:2)");
		assert_parse!(MonochromeMediaFeature, "(monochrome:8)");
		assert_parse!(MonochromeMediaFeature, "(min-monochrome:2)");
		assert_parse!(MonochromeMediaFeature, "(max-monochrome:2)");
		assert_parse!(MonochromeMediaFeature, "(monochrome<=3)");
		assert_parse!(MonochromeMediaFeature, "(monochrome>=5)");
		assert_parse!(MonochromeMediaFeature, "(monochrome>=8)");
		assert_parse!(MonochromeMediaFeature, "(monochrome=16)");
		assert_parse!(MonochromeMediaFeature, "(6=monochrome)");
		assert_parse!(MonochromeMediaFeature, "(2<=monochrome)");
		assert_parse!(MonochromeMediaFeature, "(2<monochrome<4)");
		assert_parse!(MonochromeMediaFeature, "(4>monochrome<8)");
		assert_parse!(MonochromeMediaFeature, "(4>=monochrome<=8)");
		assert_parse!(MonochromeMediaFeature, "(4<=monochrome>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(MonochromeMediaFeature, "(monochrome:)");
		assert_parse_error!(MonochromeMediaFeature, "(monochrome: > 10px)");
		assert_parse_error!(MonochromeMediaFeature, "(max-monochrome > 10px)");
		assert_parse_error!(MonochromeMediaFeature, "(min-monochrome > 10px)");
		assert_parse_error!(MonochromeMediaFeature, "(monochrome: 1px)");
		assert_parse_error!(MonochromeMediaFeature, "(monochrome: red)");
		assert_parse_error!(MonochromeMediaFeature, "(pointer: 1)");
	}
}
