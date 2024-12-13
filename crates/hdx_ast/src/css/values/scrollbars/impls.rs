pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ScrollbarColor, 280);
	}

	#[test]
	fn test_parse() {
		assert_parse!(ScrollbarColor, "red red");
		assert_parse!(ScrollbarColor, "auto");
		assert_parse!(ScrollbarColor, "red #eee", "red#eee");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(ScrollbarColor, "auto red");
		assert_parse_error!(ScrollbarColor, "red");
		assert_parse_error!(ScrollbarColor, "red green blue");
	}
}
