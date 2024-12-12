use crate::css::units::Length;
use hdx_parser::ranged_feature;

ranged_feature!(HeightMediaFeature[atom!("height")], Length);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(HeightMediaFeature, 76);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HeightMediaFeature, "height:360px");
		assert_parse!(HeightMediaFeature, "height:35rem");
		assert_parse!(HeightMediaFeature, "min-height:35rem");
		assert_parse!(HeightMediaFeature, "max-height:35rem");
		assert_parse!(HeightMediaFeature, "height<=800px");
		assert_parse!(HeightMediaFeature, "height>=1400px");
		assert_parse!(HeightMediaFeature, "height>=1400px");
		assert_parse!(HeightMediaFeature, "height=1400px");
		assert_parse!(HeightMediaFeature, "1400px=height");
		assert_parse!(HeightMediaFeature, "100px<=height");
		assert_parse!(HeightMediaFeature, "100px<height<1400px");
		assert_parse!(HeightMediaFeature, "100px>height<1400px");
		assert_parse!(HeightMediaFeature, "100px>=height<=1400px");
		assert_parse!(HeightMediaFeature, "100px<=height>1400px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HeightMediaFeature, "height:");
		assert_parse_error!(HeightMediaFeature, "height: > 10px");
		assert_parse_error!(HeightMediaFeature, "max-height > 10px");
		assert_parse_error!(HeightMediaFeature, "min-height > 10px");
		assert_parse_error!(HeightMediaFeature, "height: 1%");
		assert_parse_error!(HeightMediaFeature, "height: 1%");
		assert_parse_error!(HeightMediaFeature, "pointer: 1px");
	}
}
