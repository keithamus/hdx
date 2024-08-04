use hdx_derive::{Value, from_syntax};

// https://drafts.csswg.org/css-break/#widows-orphans
#[from_syntax(<integer [1,]>)]
#[initial(2)]
pub struct Orphans;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Orphans, 4);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Orphans, "8");
		assert_parse!(Orphans, "1");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Orphans, "8.2");
		assert_parse_error!(Orphans, "10%");
		assert_parse_error!(Orphans, "0");
	}
}
