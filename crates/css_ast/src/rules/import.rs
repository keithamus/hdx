use crate::Todo;

// https://drafts.csswg.org/css-cascade-5/#at-ruledef-import
pub type ImportRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImportRule>(), 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(ImportRule, "@import \"foo.css\"");
	}
}
