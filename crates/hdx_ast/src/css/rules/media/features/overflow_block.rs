use hdx_parser::discrete_feature;

discrete_feature!(OverflowBlockMediaFeature[atom!("overflow-block")] {
	None: atom!("none"),
	Scroll: atom!("scroll"),
	Paged: atom!("paged"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OverflowBlockMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowBlockMediaFeature, "overflow-block");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block:none");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block:scroll");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block:paged");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowBlockMediaFeature, "overflow-block:");
		assert_parse_error!(OverflowBlockMediaFeature, "overflow-block: page");
	}
}
