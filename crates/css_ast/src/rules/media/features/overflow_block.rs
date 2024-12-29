use css_parse::{discrete_feature, keyword_set};

discrete_feature!(OverflowBlockMediaFeature, "overflow-block", OverflowBlockMediaFeatureKeyword);

keyword_set!(OverflowBlockMediaFeatureKeyword { None: "none", Scroll: "scroll", Paged: "paged" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowBlockMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:none)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:scroll)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:paged)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowBlockMediaFeature, "(overflow-block:)");
		assert_parse_error!(OverflowBlockMediaFeature, "(overflow-block: page)");
	}
}
