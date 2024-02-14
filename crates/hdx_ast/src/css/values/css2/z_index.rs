#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Parsable, Writable, css::values::units::CSSFloat};

#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum ZIndex {
	#[default]
	Auto,
	#[parsable(Number, Check::Int)]
	Integer(CSSFloat),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<ZIndex>(), 8);
	}
}
