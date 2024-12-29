use css_parse::{discrete_feature, keyword_set};

discrete_feature!(AnyPointerMediaFeature, "any-pointer", AnyPointerMediaFeatureKeyword);

keyword_set!(AnyPointerMediaFeatureKeyword { None: "none", Coarse: "coarse", Fine: "fine" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnyPointerMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnyPointerMediaFeature, "(any-pointer)");
		assert_parse!(AnyPointerMediaFeature, "(any-pointer:none)");
		assert_parse!(AnyPointerMediaFeature, "(any-pointer:coarse)");
		assert_parse!(AnyPointerMediaFeature, "(any-pointer:fine)");
	}
}
