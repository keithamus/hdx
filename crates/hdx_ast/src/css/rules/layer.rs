use crate::Todo;

// https://drafts.csswg.org/css-cascade-5/#layering
pub type Layer = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Layer, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Layer, "@layer");
	}
}
