use crate::css::values::Todo;

// https://drafts.css-houdini.org/css-properties-values-api/#at-ruledef-property
pub type Property = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Property, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Property, "@property");
	}
}
