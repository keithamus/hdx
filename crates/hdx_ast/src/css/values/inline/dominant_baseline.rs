use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-dominant-baseline
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DominantBaseline {
	#[default]
	Auto, // atom!("auto")
	TextBottom,   // atom!("text-bottom")
	Alphabetic,   // atom!("alphabetic")
	Ideographic,  // atom!("ideographic")
	Middle,       // atom!("middle")
	Central,      // atom!("central")
	Mathematical, // atom!("mathematical")
	Hanging,      // atom!("hanging")
	TextTop,      // atom!("text-top")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DominantBaseline, 1);
	}
}
