use hdx_parser::discrete_feature;

discrete_feature!(OverflowInlineMediaFeature[atom!("overflow-inline")] {
	None: atom!("none"),
	Scroll: atom!("scroll"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OverflowInlineMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowInlineMediaFeature, "overflow-inline");
		assert_parse!(OverflowInlineMediaFeature, "overflow-inline:none");
		assert_parse!(OverflowInlineMediaFeature, "overflow-inline:scroll");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowInlineMediaFeature, "overflow-inline:");
		assert_parse_error!(OverflowInlineMediaFeature, "overflow-inline: page");
	}
}
