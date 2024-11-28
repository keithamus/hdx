use crate::Todo;

// https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style
pub type StartingStyle = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StartingStyle, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(StartingStyle, "@starting-style");
	}
}
