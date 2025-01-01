use css_parse::{discrete_feature, keyword_set};

discrete_feature!(PrefersReducedMotionMediaFeature, "prefers-reduced-motion", PrefersReducedMotionMediaFeatureKeyword);

keyword_set!(PrefersReducedMotionMediaFeatureKeyword { NoPreference: "no-preference", Reduce: "reduce" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersReducedMotionMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedMotionMediaFeature, "(prefers-reduced-motion)");
		assert_parse!(PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:no-preference)");
		assert_parse!(PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:reduce)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:)");
		assert_parse_error!(PrefersReducedMotionMediaFeature, "(prefers-reduced-motion: reduced)");
	}
}
