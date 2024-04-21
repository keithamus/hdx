use crate::macros::discrete_media_feature;

discrete_media_feature!(PrefersReducedTransparencyMediaFeature[atom!("prefers-reduced-transparency")] {
	NoPreference: atom!("no-preference"),
	Reduce: atom!("reduce"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersReducedTransparencyMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency: no-preference");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency: reduce");
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			PrefersReducedTransparencyMediaFeature,
			"prefers-reduced-transparency: no-preference",
			"prefers-reduced-transparency:no-preference"
		);
		assert_minify!(
			PrefersReducedTransparencyMediaFeature,
			"prefers-reduced-transparency: reduce",
			"prefers-reduced-transparency:reduce"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency:");
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "prefers-reduced-transparency: reduced");
	}
}
