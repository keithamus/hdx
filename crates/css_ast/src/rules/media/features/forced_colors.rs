use css_parse::{discrete_feature, keyword_set};

discrete_feature!(ForcedColorsMediaFeature, "forced-colors", ForcedColorsMediaFeatureKeyword);

keyword_set!(ForcedColorsMediaFeatureKeyword { None: "none", Active: "active" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ForcedColorsMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ForcedColorsMediaFeature, "(forced-colors)");
		assert_parse!(ForcedColorsMediaFeature, "(forced-colors:none)");
		assert_parse!(ForcedColorsMediaFeature, "(forced-colors:active)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ForcedColorsMediaFeature, "(forced-colors:)");
		assert_parse_error!(ForcedColorsMediaFeature, "(forced-colors: pointer)");
		assert_parse_error!(ForcedColorsMediaFeature, "(pointer: none)");
	}
}
