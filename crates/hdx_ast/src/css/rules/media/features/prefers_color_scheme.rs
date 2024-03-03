use crate::macros::discrete_media_feature;

discrete_media_feature!(PrefersColorSchemeMediaFeature[atom!("prefers-color-scheme")] {
	Light: atom!("light"),
	Dark: atom!("dark"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PrefersColorSchemeMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersColorSchemeMediaFeature, "prefers-color-scheme");
		assert_parse!(PrefersColorSchemeMediaFeature, "prefers-color-scheme: light");
		assert_parse!(PrefersColorSchemeMediaFeature, "prefers-color-scheme: dark");
	}

	#[test]
	fn test_minify() {
		assert_minify!(PrefersColorSchemeMediaFeature, "prefers-color-scheme: light", "prefers-color-scheme:light");
		assert_minify!(PrefersColorSchemeMediaFeature, "prefers-color-scheme: dark", "prefers-color-scheme:dark");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersColorSchemeMediaFeature, "prefers-color-scheme:");
		assert_parse_error!(PrefersColorSchemeMediaFeature, "prefers-color-scheme: dimmed");
	}
}
