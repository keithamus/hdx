use hdx_parser::discrete_feature;

discrete_feature!(PrefersReducedTransparencyMediaFeature[atom!("prefers-reduced-transparency")] {
	NoPreference: atom!("no-preference"),
	Reduce: atom!("reduce"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersReducedTransparencyMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency:no-preference");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency:reduce");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency:");
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency: reduced");
	}
}
