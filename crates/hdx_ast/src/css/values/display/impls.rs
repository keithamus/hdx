pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

use super::types::DisplayOutside;

impl Default for super::Display {
	fn default() -> Self {
		Self::DisplayOutsideDisplayInside(Some(DisplayOutside::Inline), None)
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(Display, 3);
	}

	#[test]
	fn test_writes() {
		// Parsing a display value should be written identically
		assert_parse!(Display, "none");
		assert_parse!(Display, "contents");
		assert_parse!(Display, "list-item");
		assert_parse!(Display, "block flow");
		assert_parse!(Display, "block flow-root");
		assert_parse!(Display, "inline flow");
		assert_parse!(Display, "inline flow-root");
		assert_parse!(Display, "run-in flow");
		assert_parse!(Display, "block flow list-item");
		assert_parse!(Display, "inline flow list-item");
		assert_parse!(Display, "block flex");
		assert_parse!(Display, "inline flex");
		assert_parse!(Display, "block grid");
		assert_parse!(Display, "inline grid");
		assert_parse!(Display, "inline ruby");
		assert_parse!(Display, "block ruby");
		assert_parse!(Display, "block table");
		assert_parse!(Display, "inline table");
	}

	#[test]
	fn test_errors() {
		// Parsing a display value should be written identically
		assert_parse_error!(Display, "none contents");
		assert_parse_error!(Display, "block contents");
		assert_parse_error!(Display, "list-item table");
		assert_parse_error!(Display, "list-item flex");
		assert_parse_error!(Display, "list-item grid");
		assert_parse_error!(Display, "list-item ruby");
		assert_parse_error!(Display, "ruby list-item");
		assert_parse_error!(Display, "block block");
		assert_parse_error!(Display, "flow flow-root");
		assert_parse_error!(Display, "inline inline");
		assert_parse_error!(Display, "inline inline-table");
		assert_parse_error!(Display, "block inline-grid");
	}
}
