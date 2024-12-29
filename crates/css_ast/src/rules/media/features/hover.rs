use css_parse::{discrete_feature, keyword_set};

discrete_feature!(HoverMediaFeature, "hover", HoverMediaFeatureKeyword);

keyword_set!(HoverMediaFeatureKeyword { None: "none", Hover: "hover" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HoverMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HoverMediaFeature, "(hover)");
		assert_parse!(HoverMediaFeature, "(hover:hover)");
		assert_parse!(HoverMediaFeature, "(hover:none)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HoverMediaFeature, "(hover:)");
		assert_parse_error!(HoverMediaFeature, "(hover: hoover)");
	}
}
