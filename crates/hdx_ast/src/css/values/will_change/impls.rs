pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod test {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn test_writes() {
		assert_parse!(WillChange, "auto");
		assert_parse!(WillChange, "foo, bar, baz");
		assert_parse!(WillChange, "-webkit-perspective");
		assert_parse!(WillChange, "transform, filter, mask");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WillChange, "auto auto"); // two autos is illegal
		assert_parse_error!(WillChange, ""); // must be at-least-one
		assert_parse_error!(WillChange, "transform filter"); // no commas
		assert_parse_error!(WillChange, "0px 3px"); // dimensions not idents
	}
}
