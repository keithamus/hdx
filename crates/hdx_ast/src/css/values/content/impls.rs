pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		// assert_size!(ContentStyleValue, 1);
		// assert_size!(QuotesStyleValue, 1);
		// assert_size!(StringSetStyleValue, 1);
		assert_size!(BookmarkLevelStyleValue, 16);
		// assert_size!(BookmarkLabelStyleValue, 1);
		assert_size!(BookmarkStateStyleValue, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BookmarkLevelStyleValue, "1");
		assert_parse!(BookmarkStateStyleValue, "open");
	}
}
