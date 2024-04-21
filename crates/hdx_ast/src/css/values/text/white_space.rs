use crate::{Atomizable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[atomizable(FromToken)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WhiteSpace {
	#[default]
	Normal, // atom!("normal")
	Pre,         // atom!("pre")
	Nowrap,      // atom!("nowrap")
	PreWrap,     // atom!("pre-wrap")
	BreakSpaces, // atom!("break-spaces")
	PreLine,     // atom!("pre-line")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WhiteSpace, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WhiteSpace, "normal");
		assert_parse!(WhiteSpace, "nowrap");
		assert_parse!(WhiteSpace, "pre-wrap");
	}
}
