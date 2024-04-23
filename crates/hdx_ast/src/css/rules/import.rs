use crate::css::values::Todo;

// https://drafts.csswg.org/css-cascade-5/#at-ruledef-import
pub type Import = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Import, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Import, "@import \"foo.css\"");
	}
}
