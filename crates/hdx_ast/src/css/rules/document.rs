use crate::Todo;

// https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document
pub type Document = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Document, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Document, "@document url("") {}");
	}
}
