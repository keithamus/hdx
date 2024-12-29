use crate::Todo;

// https://drafts.csswg.org/css-fonts/#at-ruledef-font-palette-values
pub type FontPaletteValuesRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontPaletteValuesRule>(), 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontPaletteValuesRule, "@font-palette-values --cooler {}");
	}
}
