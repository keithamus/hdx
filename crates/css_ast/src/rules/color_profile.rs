use crate::Todo;

// https://drafts.csswg.org/css-color-5/#at-profile
pub type ColorProfileRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorProfileRule>(), 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(ColorProfileRule, "@color-profile --swop5c {\n\tsrc: url(\"https://example.org/SWOP2006_Coated5v2.icc\");}");
	}
}
