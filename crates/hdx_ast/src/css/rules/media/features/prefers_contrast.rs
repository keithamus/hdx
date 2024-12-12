use hdx_parser::discrete_feature;

discrete_feature!(PrefersContrastMediaFeature[atom!("prefers-contrast")] {
	NoPreference: atom!("no-preference"),
	Less: atom!("less"),
	More: atom!("more"),
	Custom: atom!("custom"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersContrastMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersContrastMediaFeature, "prefers-contrast");
		assert_parse!(PrefersContrastMediaFeature, "prefers-contrast:no-preference");
		assert_parse!(PrefersContrastMediaFeature, "prefers-contrast:less");
		assert_parse!(PrefersContrastMediaFeature, "prefers-contrast:more");
		assert_parse!(PrefersContrastMediaFeature, "prefers-contrast:custom");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersContrastMediaFeature, "prefers-contrast:");
		assert_parse_error!(PrefersContrastMediaFeature, "prefers-contrast: no-pref");
	}
}
