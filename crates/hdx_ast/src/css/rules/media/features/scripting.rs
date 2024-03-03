#[cfg(feature = "serde")]
use crate::macros::discrete_media_feature;

discrete_media_feature!(ScriptingMediaFeature[atom!("scripting")] {
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
		assert_size!(ScriptingMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScriptingMediaFeature, "scripting");
		assert_parse!(ScriptingMediaFeature, "scripting: none");
		assert_parse!(ScriptingMediaFeature, "scripting: initial-only");
		assert_parse!(ScriptingMediaFeature, "scripting: enabled");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ScriptingMediaFeature, "scripting: none", "scripting:none");
		assert_minify!(ScriptingMediaFeature, "scripting: initial-only", "scripting:initial-only");
		assert_minify!(ScriptingMediaFeature, "scripting: enabled", "scripting:enabled");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScriptingMediaFeature, "scripting:");
		assert_parse_error!(ScriptingMediaFeature, "scripting: yes");
	}
}
