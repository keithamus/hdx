use crate::Todo;

// https://drafts.csswg.org/css-contain-3/#container-rule
pub type Container = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Container, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(Container, "@container (width > 400px) { h2 { font-size; 1.5rem } }");
	}
}
