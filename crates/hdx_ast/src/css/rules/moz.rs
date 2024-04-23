use crate::css::values::Todo;

pub type MozDocument = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MozDocument, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(MozDocument, "@-moz-document");
	}
}
