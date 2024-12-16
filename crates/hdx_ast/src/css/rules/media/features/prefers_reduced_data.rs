use hdx_parser::discrete_feature;

discrete_feature!(PrefersReducedDataMediaFeature[atom!("prefers-reduced-data")] {
	NoPreference: atom!("no-preference"),
	Reduce: atom!("reduce"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersReducedDataMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data");
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data:no-preference");
		assert_parse!(PrefersReducedDataMediaFeature, "prefers-reduced-data:reduce");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedDataMediaFeature, "prefers-reduced-data:");
		assert_parse_error!(PrefersReducedDataMediaFeature, "prefers-reduced-data: reduced");
	}
}
