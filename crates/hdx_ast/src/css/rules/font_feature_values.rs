use crate::Todo;

// https://drafts.csswg.org/css-fonts/#at-ruledef-font-feature-values
pub type FontFeatureValues = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFeatureValues, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFeatureValues, "@font-feature-values Taisho Gothic {}");
	}
}
