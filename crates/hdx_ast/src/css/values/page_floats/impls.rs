pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FloatReference, 1);
		assert_size!(Float, 16);
		assert_size!(Clear, 1);
		assert_size!(FloatDefer, 8);
		assert_size!(FloatOffset, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Float, "snap-block(1px, near)");
		assert_parse!(Float, "snap-inline(1px, near)");
	}
}
