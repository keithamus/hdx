use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxDecorationBreak {
	#[default]
	Slice, // atom!("slice")
	Clone, // atom!("clone")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BoxDecorationBreak, 1);
	}
}
