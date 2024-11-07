pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;


#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(Position, 1);
		assert_size!(Top, 8);
		assert_size!(Right, 8);
		assert_size!(Bottom, 8);
		assert_size!(Left, 8);
		assert_size!(InsetBlockStart, 8);
		assert_size!(InsetInlineStart, 8);
		assert_size!(InsetBlockEnd, 8);
		assert_size!(InsetInlineEnd, 8);
		assert_size!(InsetBlock, 16);
		assert_size!(InsetInline, 16);
		assert_size!(Inset, 32);
		assert_size!(Overlay, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Position, "sticky");
		assert_parse!(InsetBlockStart, "auto");
		assert_parse!(Inset, "1px 2px");
		assert_parse!(Inset, "1px 2px 3px 4px");
	}
}
