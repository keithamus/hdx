pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(WillChangeStyleValue, "auto");
		assert_parse!(WillChangeStyleValue, "foo,bar,baz");
		assert_parse!(WillChangeStyleValue, "-webkit-perspective");
		assert_parse!(WillChangeStyleValue, "transform,filter,mask");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WillChangeStyleValue, "auto auto"); // two autos is illegal
		assert_parse_error!(WillChangeStyleValue, ""); // must be at-least-one
		assert_parse_error!(WillChangeStyleValue, "transform filter"); // no commas
		assert_parse_error!(WillChangeStyleValue, "0px 3px"); // dimensions not idents
	}
}
