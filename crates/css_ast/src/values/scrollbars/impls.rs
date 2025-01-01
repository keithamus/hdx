pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScrollbarColorStyleValue>(), 320);
	}

	#[test]
	fn test_parse() {
		assert_parse!(ScrollbarColorStyleValue, "red red");
		assert_parse!(ScrollbarColorStyleValue, "auto");
		assert_parse!(ScrollbarColorStyleValue, "red #eee", "red#eee");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(ScrollbarColorStyleValue, "auto red");
		assert_parse_error!(ScrollbarColorStyleValue, "red");
		assert_parse_error!(ScrollbarColorStyleValue, "red green blue");
	}
}
