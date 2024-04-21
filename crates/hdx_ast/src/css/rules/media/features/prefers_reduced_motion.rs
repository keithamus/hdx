use crate::macros::discrete_media_feature;

discrete_media_feature!(PrefersReducedMotionMediaFeature[atom!("prefers-reduced-motion")] {
	NoPreference: atom!("no-preference"),
	Reduce: atom!("reduce"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersReducedMotionMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion");
		assert_parse!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion: no-preference");
		assert_parse!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion: reduce");
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			PrefersReducedMotionMediaFeature,
			"prefers-reduced-motion: no-preference",
			"prefers-reduced-motion:no-preference"
		);
		assert_minify!(
			PrefersReducedMotionMediaFeature,
			"prefers-reduced-motion: reduce",
			"prefers-reduced-motion:reduce"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion:");
		assert_parse_error!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion: reduced");
	}
}
