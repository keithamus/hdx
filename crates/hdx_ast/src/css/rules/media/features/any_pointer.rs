use crate::macros::discrete_media_feature;

discrete_media_feature!(AnyPointerMediaFeature[atom!("any-pointer")] {
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
		assert_size!(AnyPointerMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnyPointerMediaFeature, "any-pointer");
		assert_parse!(AnyPointerMediaFeature, "any-pointer: none");
		assert_parse!(AnyPointerMediaFeature, "any-pointer: coarse");
		assert_parse!(AnyPointerMediaFeature, "any-pointer: fine");
	}

	#[test]
	fn test_minify() {
		assert_minify!(AnyPointerMediaFeature, "any-pointer", "any-pointer");
		assert_minify!(AnyPointerMediaFeature, "any-pointer: none", "any-pointer:none");
		assert_minify!(AnyPointerMediaFeature, "any-pointer: coarse", "any-pointer:coarse");
		assert_minify!(AnyPointerMediaFeature, "any-pointer: fine", "any-pointer:fine");
	}
}
