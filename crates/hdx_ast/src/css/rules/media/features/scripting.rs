use hdx_parser::discrete_feature;

discrete_feature!(ScriptingMediaFeature[atom!("scripting")] {
	None: atom!("none"),
	InitialOnly: atom!("initial-only"),
	Enabled: atom!("enabled"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ScriptingMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScriptingMediaFeature, "scripting");
		assert_parse!(ScriptingMediaFeature, "scripting:none");
		assert_parse!(ScriptingMediaFeature, "scripting:initial-only");
		assert_parse!(ScriptingMediaFeature, "scripting:enabled");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScriptingMediaFeature, "scripting:");
		assert_parse_error!(ScriptingMediaFeature, "scripting: yes");
	}
}
