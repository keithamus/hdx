use crate::css::values::Todo;

// https://drafts.csswg.org/css-namespaces/#at-ruledef-namespace
pub type Namespace = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Namespace, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Namespace, "@namespace");
	}
}
