use css_parse::{discrete_feature, keyword_set};

discrete_feature!(InvertedColorsMediaFeature, "inverted-colors", InvertedColorsMediaFeatureKeyword);

keyword_set!(InvertedColorsMediaFeatureKeyword { None: "none", Inverted: "inverted" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<InvertedColorsMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors)");
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors:inverted)");
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors:none)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(InvertedColorsMediaFeature, "(inverted-colors:)");
		assert_parse_error!(InvertedColorsMediaFeature, "(inverted-colors: invited)");
	}
}
