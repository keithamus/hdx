use crate::{css::units::CSSInt, macros::ranged_media_feature};

ranged_media_feature!(ColorIndexMediaFeature[atom!("color-index")], CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ColorIndexMediaFeature, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorIndexMediaFeature, "color-index: 2");
		assert_parse!(ColorIndexMediaFeature, "color-index: 8");
		assert_parse!(ColorIndexMediaFeature, "min-color-index: 2");
		assert_parse!(ColorIndexMediaFeature, "max-color-index: 2");
		assert_parse!(ColorIndexMediaFeature, "color-index <= 3");
		assert_parse!(ColorIndexMediaFeature, "color-index >= 5");
		assert_parse!(ColorIndexMediaFeature, "color-index >= 8");
		assert_parse!(ColorIndexMediaFeature, "color-index = 16");
		assert_parse!(ColorIndexMediaFeature, "6 = color-index", "color-index = 6");
		assert_parse!(ColorIndexMediaFeature, "2 <= color-index", "color-index <= 2");
		assert_parse!(ColorIndexMediaFeature, "2 < color-index < 4");
		assert_parse!(ColorIndexMediaFeature, "4 > color-index < 8");
		assert_parse!(ColorIndexMediaFeature, "4 >= color-index <= 8");
		assert_parse!(ColorIndexMediaFeature, "4 <= color-index > 8");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ColorIndexMediaFeature, "color-index: 8", "color-index:8");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorIndexMediaFeature, "1 = color-index = 8");
		assert_parse_error!(ColorIndexMediaFeature, "color-index:");
		assert_parse_error!(ColorIndexMediaFeature, "color-index: > 10px");
		assert_parse_error!(ColorIndexMediaFeature, "max-color-index > 10px");
		assert_parse_error!(ColorIndexMediaFeature, "min-color-index > 10px");
		assert_parse_error!(ColorIndexMediaFeature, "color-index: 1px");
		assert_parse_error!(ColorIndexMediaFeature, "color-index: red");
		assert_parse_error!(ColorIndexMediaFeature, "pointer: 1");
	}
}
