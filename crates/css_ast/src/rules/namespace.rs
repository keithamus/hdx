use crate::Todo;

// https://drafts.csswg.org/css-namespaces/#at-ruledef-namespace
pub type NamespaceRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NamespaceRule>(), 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(NamespaceRule, "@namespace");
	}
}
