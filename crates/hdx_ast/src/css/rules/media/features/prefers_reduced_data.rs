use crate::macros::discrete_media_feature;

discrete_media_feature!(PrefersReducedDataMediaFeature[atom!("prefers-reduced-data")] {
	NoPreference: atom!("no-preference"),
	Reduce: atom!("reduce"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersReducedDataMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data");
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data: no-preference");
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data: reduce");
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			PrefersReducedDataMediaFeature,
			"prefers-reduced-data: no-preference",
			"prefers-reduced-data:no-preference"
		);
		assert_minify!(PrefersReducedDataMediaFeature, "prefers-reduced-data: reduce", "prefers-reduced-data:reduce");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedDataMediaFeature, "prefers-reduced-data:");
		assert_parse_error!(PrefersReducedDataMediaFeature, "prefers-reduced-data: reduced");
	}
}
