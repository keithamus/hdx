use crate::Todo;

// https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style
pub type StartingStyleRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StartingStyleRule, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(StartingStyleRule, "@starting-style");
	}
}
