#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextDecorationStyle {
	#[default]
	Solid, // atom!("solid"),
	Double, // atom!("double")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Wavy,   // atom!("wavy")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<TextDecorationStyle>(), 1);
	}
}
