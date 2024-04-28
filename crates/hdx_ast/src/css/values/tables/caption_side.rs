use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-tables-3/#propdef-caption-side
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum CaptionSide {
	#[default]
	Top, // atom!("top")
	Bottom, // atom!("bottom")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CaptionSide, 1);
	}
}
