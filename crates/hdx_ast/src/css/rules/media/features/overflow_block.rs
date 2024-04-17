use crate::macros::discrete_media_feature;

discrete_media_feature!(OverflowBlockMediaFeature[atom!("overflow-block")] {
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
		assert_size!(OverflowBlockMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowBlockMediaFeature, "overflow-block");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block: none");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block: scroll");
		assert_parse!(OverflowBlockMediaFeature, "overflow-block: paged");
	}

	#[test]
	fn test_minify() {
		assert_minify!(OverflowBlockMediaFeature, "overflow-block: none", "overflow-block:none");
		assert_minify!(OverflowBlockMediaFeature, "overflow-block: scroll", "overflow-block:scroll");
		assert_minify!(OverflowBlockMediaFeature, "overflow-block: paged", "overflow-block:paged");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowBlockMediaFeature, "overflow-block:");
		assert_parse_error!(OverflowBlockMediaFeature, "overflow-block: page");
	}
}
