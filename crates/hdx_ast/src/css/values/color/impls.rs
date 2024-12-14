pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ColorStyleValue, 160);
		assert_size!(OpacityStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorStyleValue, "red");
		assert_parse!(OpacityStyleValue, "1");
	}
}
