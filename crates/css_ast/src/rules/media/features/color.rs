use crate::units::CSSInt;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(ColorMediaFeatureKeyword { Color: "color", MaxColor: "max-color", MinColor: "min-color" });

impl RangedFeatureKeyword for ColorMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxColor(_) | Self::MinColor(_))
	}
}

ranged_feature!(ColorMediaFeature, ColorMediaFeatureKeyword, CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorMediaFeature>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorMediaFeature, "(color:2)");
		assert_parse!(ColorMediaFeature, "(color:8)");
		assert_parse!(ColorMediaFeature, "(min-color:2)");
		assert_parse!(ColorMediaFeature, "(max-color:2)");
		assert_parse!(ColorMediaFeature, "(color<=3)");
		assert_parse!(ColorMediaFeature, "(color>=5)");
		assert_parse!(ColorMediaFeature, "(color>=8)");
		assert_parse!(ColorMediaFeature, "(color=16)");
		assert_parse!(ColorMediaFeature, "(6=color)");
		assert_parse!(ColorMediaFeature, "(2<=color)");
		assert_parse!(ColorMediaFeature, "(2<color<4)");
		assert_parse!(ColorMediaFeature, "(4>color<8)");
		assert_parse!(ColorMediaFeature, "(4>=color<=8)");
		assert_parse!(ColorMediaFeature, "(4<=color>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorMediaFeature, "(color:)");
		assert_parse_error!(ColorMediaFeature, "(color: > 10px)");
		assert_parse_error!(ColorMediaFeature, "(max-color > 10px)");
		assert_parse_error!(ColorMediaFeature, "(min-color > 10px)");
		assert_parse_error!(ColorMediaFeature, "(color: 1px)");
		assert_parse_error!(ColorMediaFeature, "(color: red)");
		assert_parse_error!(ColorMediaFeature, "(pointer: 1)");
	}
}
