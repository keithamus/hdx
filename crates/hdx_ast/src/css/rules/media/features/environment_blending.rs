use hdx_parser::discrete_feature;

discrete_feature!(EnvironmentBlendingMediaFeature[atom!("environment-blending")] {
	Opaque: atom!("opaque"),
	Additive: atom!("additive"),
	Subtractive: atom!("subtractive"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(EnvironmentBlendingMediaFeature, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EnvironmentBlendingMediaFeature, "environment-blending");
		assert_parse!(EnvironmentBlendingMediaFeature, "environment-blending:opaque");
		assert_parse!(EnvironmentBlendingMediaFeature, "environment-blending:additive");
		assert_parse!(EnvironmentBlendingMediaFeature, "environment-blending:subtractive");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(EnvironmentBlendingMediaFeature, "environment-blending:");
		assert_parse_error!(EnvironmentBlendingMediaFeature, "environment-blending: pointer");
		assert_parse_error!(EnvironmentBlendingMediaFeature, "pointer: subtractive");
	}
}
