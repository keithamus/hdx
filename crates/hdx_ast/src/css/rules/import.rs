use crate::Todo;

// https://drafts.csswg.org/css-cascade-5/#at-ruledef-import
pub type ImportRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ImportRule, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(ImportRule, "@import \"foo.css\"");
	}
}
