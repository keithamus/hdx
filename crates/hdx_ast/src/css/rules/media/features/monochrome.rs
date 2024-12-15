use crate::css::units::CSSInt;
use hdx_parser::ranged_feature;

ranged_feature!(MonochromeMediaFeature[atom!("monochrome")], CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MonochromeMediaFeature, 92);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MonochromeMediaFeature, "monochrome:2");
		assert_parse!(MonochromeMediaFeature, "monochrome:8");
		assert_parse!(MonochromeMediaFeature, "min-monochrome:2");
		assert_parse!(MonochromeMediaFeature, "max-monochrome:2");
		assert_parse!(MonochromeMediaFeature, "monochrome<=3");
		assert_parse!(MonochromeMediaFeature, "monochrome>=5");
		assert_parse!(MonochromeMediaFeature, "monochrome>=8");
		assert_parse!(MonochromeMediaFeature, "monochrome=16");
		assert_parse!(MonochromeMediaFeature, "6=monochrome");
		assert_parse!(MonochromeMediaFeature, "2<=monochrome");
		assert_parse!(MonochromeMediaFeature, "2<monochrome<4");
		assert_parse!(MonochromeMediaFeature, "4>monochrome<8");
		assert_parse!(MonochromeMediaFeature, "4>=monochrome<=8");
		assert_parse!(MonochromeMediaFeature, "4<=monochrome>8");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(MonochromeMediaFeature, "monochrome:");
		assert_parse_error!(MonochromeMediaFeature, "monochrome: > 10px");
		assert_parse_error!(MonochromeMediaFeature, "max-monochrome > 10px");
		assert_parse_error!(MonochromeMediaFeature, "min-monochrome > 10px");
		assert_parse_error!(MonochromeMediaFeature, "monochrome: 1px");
		assert_parse_error!(MonochromeMediaFeature, "monochrome: red");
		assert_parse_error!(MonochromeMediaFeature, "pointer: 1");
	}
}
