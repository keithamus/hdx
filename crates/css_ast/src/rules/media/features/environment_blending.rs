use css_parse::{discrete_feature, keyword_set};

discrete_feature!(EnvironmentBlendingMediaFeature, "environment-blending", EnvironmentBlendingMediaFeatureKeyword);

keyword_set!(EnvironmentBlendingMediaFeatureKeyword {
	Opaque: "opaque",
	Additive: "additive",
	Subtractive: "subtractive",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<EnvironmentBlendingMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EnvironmentBlendingMediaFeature, "(environment-blending)");
		assert_parse!(EnvironmentBlendingMediaFeature, "(environment-blending:opaque)");
		assert_parse!(EnvironmentBlendingMediaFeature, "(environment-blending:additive)");
		assert_parse!(EnvironmentBlendingMediaFeature, "(environment-blending:subtractive)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(EnvironmentBlendingMediaFeature, "(environment-blending:)");
		assert_parse_error!(EnvironmentBlendingMediaFeature, "(environment-blending: pointer)");
		assert_parse_error!(EnvironmentBlendingMediaFeature, "(pointer: subtractive)");
	}
}
