use hdx_parser::discrete_feature;

discrete_feature!(PointerMediaFeature[atom!("pointer")] {
	None: atom!("none"),
	Coarse: atom!("coarse"),
	Fine: atom!("fine"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PointerMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PointerMediaFeature, "pointer");
		assert_parse!(PointerMediaFeature, "pointer:none");
		assert_parse!(PointerMediaFeature, "pointer:coarse");
		assert_parse!(PointerMediaFeature, "pointer:fine");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PointerMediaFeature, "pointer:");
		assert_parse_error!(PointerMediaFeature, "pointer: pointer");
	}
}
