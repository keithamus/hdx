use crate::Todo;

// https://drafts.csswg.org/css-cascade-6/#at-ruledef-scope
pub type Scope = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Scope, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Scope, "@scope");
	}
}
