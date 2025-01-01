use crate::units::Length;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(HeightMediaFeatureKeyword { Height: "height", MaxHeight: "max-height", MinHeight: "min-height" });

impl RangedFeatureKeyword for HeightMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxHeight(_) | Self::MinHeight(_))
	}
}

ranged_feature!(HeightMediaFeature, HeightMediaFeatureKeyword, Length);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HeightMediaFeature>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HeightMediaFeature, "(height:360px)");
		assert_parse!(HeightMediaFeature, "(height:35rem)");
		assert_parse!(HeightMediaFeature, "(min-height:35rem)");
		assert_parse!(HeightMediaFeature, "(max-height:35rem)");
		assert_parse!(HeightMediaFeature, "(height<=800px)");
		assert_parse!(HeightMediaFeature, "(height>=1400px)");
		assert_parse!(HeightMediaFeature, "(height>=1400px)");
		assert_parse!(HeightMediaFeature, "(height=1400px)");
		assert_parse!(HeightMediaFeature, "(1400px=height)");
		assert_parse!(HeightMediaFeature, "(100px<=height)");
		assert_parse!(HeightMediaFeature, "(100px<height<1400px)");
		assert_parse!(HeightMediaFeature, "(100px>height<1400px)");
		assert_parse!(HeightMediaFeature, "(100px>=height<=1400px)");
		assert_parse!(HeightMediaFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HeightMediaFeature, "(height:)");
		assert_parse_error!(HeightMediaFeature, "(height: > 10px)");
		assert_parse_error!(HeightMediaFeature, "(max-height > 10px)");
		assert_parse_error!(HeightMediaFeature, "(min-height > 10px)");
		assert_parse_error!(HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(HeightMediaFeature, "(pointer: 1px)");
	}
}
