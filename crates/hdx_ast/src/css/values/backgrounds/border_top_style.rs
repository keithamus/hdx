#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-backgrounds-3/#propdef-border-top-style
#[derive(Parsable, Writable, Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BorderTopStyle {
	#[default]
	None, // atom!("none")
	Hidden, // atom!("hidden")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Solid,  // atom!("solid")
	Double, // atom!("double")
	Groove, // atom!("groove")
	Ridge,  // atom!("ridge")
	Inset,  // atom!("inset")
	Outset, // atom!("outset")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderTopStyle, 1);
	}
}
