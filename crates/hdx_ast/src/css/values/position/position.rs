use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-position-3/#propdef-position
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Position {
	#[default]
	Static, // atom!("static")
	Relative, // atom!("relative")
	Absolute, // atom!("absolute")
	Sticky,   // atom!("sticky")
	Fixed,    // atom!("fixed")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Position, 1);
	}
}
