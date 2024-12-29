use css_parse::{discrete_feature, keyword_set};

discrete_feature!(OverflowInlineMediaFeature, "overflow-inline", OverflowInlineMediaFeatureKeyword);

keyword_set!(OverflowInlineMediaFeatureKeyword { None: "none", Scroll: "scroll" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowInlineMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowInlineMediaFeature, "(overflow-inline)");
		assert_parse!(OverflowInlineMediaFeature, "(overflow-inline:none)");
		assert_parse!(OverflowInlineMediaFeature, "(overflow-inline:scroll)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowInlineMediaFeature, "(overflow-inline:)");
		assert_parse_error!(OverflowInlineMediaFeature, "(overflow-inline: page)");
	}
}
