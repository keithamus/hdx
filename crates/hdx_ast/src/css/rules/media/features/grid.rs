use hdx_parser::bool_feature;

bool_feature!(GridMediaFeature[atom!("grid")]);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(GridMediaFeature, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridMediaFeature, "grid:1");
		assert_parse!(GridMediaFeature, "grid");
	}
}
