use crate::Todo;

// https://drafts.csswg.org/css-cascade-6/#at-ruledef-scope
pub type ScopeRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ScopeRule, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(ScopeRule, "@scope");
	}
}
