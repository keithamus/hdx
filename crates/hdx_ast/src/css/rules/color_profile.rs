use crate::css::values::Todo;

// https://drafts.csswg.org/css-color-5/#at-profile
pub type ColorProfile = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ColorProfile, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(ColorProfile, "@color-profile --swop5c {\n\tsrc: url(\"https://example.org/SWOP2006_Coated5v2.icc\");}");
	}
}
