pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Color, 36);
		assert_size!(Opacity, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Color, "red");
		assert_parse!(Opacity, "1");
	}
}
