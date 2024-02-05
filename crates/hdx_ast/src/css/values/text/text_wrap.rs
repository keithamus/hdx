#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-text-wrap
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextWrap {
	#[default]
	Wrap, // atom!("wrap")
	Nowrap,  // atom!("nowrap")
	Balance, // atom!("balance")
	Stable,  // atom!("stable")
	Pretty,  // atom!("pretty")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<TextWrap>(), 1);
	}
}
