use crate::macros::bool_media_feature;

bool_media_feature!(GridMediaFeature[atom!("grid")]);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(GridMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridMediaFeature, "grid: 1");
		assert_parse!(GridMediaFeature, "grid", "grid: 0");
	}

	#[test]
	fn test_minify() {
		assert_minify!(GridMediaFeature, "grid: 1", "grid:1");
		assert_minify!(GridMediaFeature, "grid: 0", "grid");
	}
}
