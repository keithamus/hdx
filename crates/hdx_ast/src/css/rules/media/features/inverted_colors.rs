use hdx_parser::discrete_feature;

discrete_feature!(InvertedColorsMediaFeature[atom!("inverted-colors")] {
	None: atom!("none"),
	Inverted: atom!("inverted"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(InvertedColorsMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(InvertedColorsMediaFeature, "inverted-colors");
		assert_parse!(InvertedColorsMediaFeature, "inverted-colors:inverted");
		assert_parse!(InvertedColorsMediaFeature, "inverted-colors:none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(InvertedColorsMediaFeature, "inverted-colors:");
		assert_parse_error!(InvertedColorsMediaFeature, "inverted-colors: invited");
	}
}
