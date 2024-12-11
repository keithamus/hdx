use crate::Todo;

// https://drafts.csswg.org/css-fonts/#at-ruledef-font-feature-values
pub type FontFeatureValuesRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFeatureValuesRule, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFeatureValuesRule, "@font-feature-values Taisho Gothic {}");
	}
}
