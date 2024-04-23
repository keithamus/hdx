use crate::css::values::Todo;

// https://drafts.csswg.org/css-fonts/#font-face-rule
pub type FontFace = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFace, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFace, "@font-face {}");
	}
}
