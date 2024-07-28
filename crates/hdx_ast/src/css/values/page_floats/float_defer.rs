use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::CSSFloat;

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-defer
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FloatDefer, 8);
	}
}
