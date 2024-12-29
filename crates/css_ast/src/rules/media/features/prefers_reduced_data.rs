use css_parse::{discrete_feature, keyword_set};

discrete_feature!(PrefersReducedDataMediaFeature, "prefers-reduced-data", PrefersReducedDataMediaFeatureKeyword);

keyword_set!(PrefersReducedDataMediaFeatureKeyword { NoPreference: "no-preference", Reduce: "reduce" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersReducedDataMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data)");
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:no-preference)");
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:reduce)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:)");
		assert_parse_error!(PrefersReducedDataMediaFeature, "(prefers-reduced-data: reduced)");
	}
}
