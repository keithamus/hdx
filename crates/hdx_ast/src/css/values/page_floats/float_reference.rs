#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-reference
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FloatReference {
	#[default]
	Inline, // atom!("inline")
	Column, // atom!("column")
	Region, // atom!("region")
	Page,   // atom!("page")
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FloatReference, 1);
	}
}
