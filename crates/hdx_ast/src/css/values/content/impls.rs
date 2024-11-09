pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		// assert_size!(Content, 1);
		// assert_size!(Quotes, 1);
		// assert_size!(StringSet, 1);
		assert_size!(BookmarkLevel, 8);
		// assert_size!(BookmarkLabel, 1);
		assert_size!(BookmarkState, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BookmarkLevel, "1");
		assert_parse!(BookmarkState, "open");
	}
}
