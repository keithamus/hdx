use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-text-align
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextAlign {
	#[default]
	Start, // atom!("start")
	End,         // atom!("end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Center,      // atom!("center")
	Justify,     // atom!("justify")
	MatchParent, // atom!("match-parent")
	JustifyAll,  /* atom!("justify-all")
	              * TODO: Custom? */
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextAlign, 1);
	}
}
