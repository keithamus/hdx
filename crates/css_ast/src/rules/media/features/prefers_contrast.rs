use css_parse::{discrete_feature, keyword_set};

discrete_feature!(PrefersContrastMediaFeature, "prefers-contrast", PrefersContrastMediaFeatureKeyword);

keyword_set!(PrefersContrastMediaFeatureKeyword {
	NoPreference: "no-preference",
	Less: "less",
	More: "more",
	Custom: "custom",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersContrastMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersContrastMediaFeature, "(prefers-contrast)");
		assert_parse!(PrefersContrastMediaFeature, "(prefers-contrast:no-preference)");
		assert_parse!(PrefersContrastMediaFeature, "(prefers-contrast:less)");
		assert_parse!(PrefersContrastMediaFeature, "(prefers-contrast:more)");
		assert_parse!(PrefersContrastMediaFeature, "(prefers-contrast:custom)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersContrastMediaFeature, "(prefers-contrast:)");
		assert_parse_error!(PrefersContrastMediaFeature, "(prefers-contrast: no-pref)");
	}
}
