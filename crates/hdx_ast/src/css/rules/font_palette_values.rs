use crate::css::values::Todo;

// https://drafts.csswg.org/css-fonts/#at-ruledef-font-palette-values
pub type FontPaletteValues = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontPaletteValues, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontPaletteValues, "@font-palette-values --cooler {}");
	}
}
