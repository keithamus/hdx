use css_parse::boolean_feature;

boolean_feature!(GridMediaFeature, "grid");

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GridMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridMediaFeature, "(grid:1)");
		assert_parse!(GridMediaFeature, "(grid)");
	}
}
