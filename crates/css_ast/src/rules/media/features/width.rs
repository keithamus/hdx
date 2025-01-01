use crate::units::Length;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(WidthMediaFeatureKeyword { Width: "width", MaxWidth: "max-width", MinWidth: "min-width" });

impl RangedFeatureKeyword for WidthMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxWidth(_) | Self::MinWidth(_))
	}
}

ranged_feature!(WidthMediaFeature, WidthMediaFeatureKeyword, Length);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthMediaFeature>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthMediaFeature, "(width:360px)");
		assert_parse!(WidthMediaFeature, "(width:35rem)");
		assert_parse!(WidthMediaFeature, "(min-width:35rem)");
		assert_parse!(WidthMediaFeature, "(max-width:35rem)");
		assert_parse!(WidthMediaFeature, "(width<=800px)");
		assert_parse!(WidthMediaFeature, "(width>=1400px)");
		assert_parse!(WidthMediaFeature, "(width>=1400px)");
		assert_parse!(WidthMediaFeature, "(width=1400px)");
		assert_parse!(WidthMediaFeature, "(1400px=width)");
		assert_parse!(WidthMediaFeature, "(100px<=width)");
		assert_parse!(WidthMediaFeature, "(100px<width<1400px)");
		assert_parse!(WidthMediaFeature, "(100px>width<1400px)");
		assert_parse!(WidthMediaFeature, "(100px>=width<=1400px)");
		assert_parse!(WidthMediaFeature, "(100px<=width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WidthMediaFeature, "(width:)");
		assert_parse_error!(WidthMediaFeature, "(width: > 10px)");
		assert_parse_error!(WidthMediaFeature, "(max-width > 10px)");
		assert_parse_error!(WidthMediaFeature, "(min-width > 10px)");
		assert_parse_error!(WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(WidthMediaFeature, "(pointer: 1px)");
	}
}
