use crate::Todo;

// https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style
pub type StartingStyleRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StartingStyleRule>(), 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(StartingStyleRule, "@starting-style");
	}
}
