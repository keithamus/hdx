#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Parsable, Writable};

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-defer
#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FloatDefer {
	#[default]
	Last,
	None,
	#[parsable(Number, Check::Int)]
	Integer(CSSFloat),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<FloatDefer>(), 8);
	}
}
