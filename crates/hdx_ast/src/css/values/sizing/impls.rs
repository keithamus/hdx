pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;


#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Width, 12);
		assert_size!(Height, 12);
		assert_size!(MinWidth, 12);
		assert_size!(MinHeight, 12);
		assert_size!(MaxWidth, 12);
		assert_size!(MaxHeight, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Width, "0");
		assert_parse!(Width, "1px");
		assert_parse!(Width, "fit-content");
		assert_parse!(Width, "fit-content(20rem)");
		assert_parse!(Width, "fit-content(0)");
	}
}
