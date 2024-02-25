#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Parsable, Writable};

#[derive(Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Zoom {
	#[default]
	Normal, // atom!("normal")
	Reset, // atom!("reset")
	#[parsable(Number)]
	Number(CSSFloat),
	#[parsable(Dimension, atom = "%")]
	#[writable(suffix = "%")]
	Percent(CSSFloat),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Zoom>(), 8);
	}
}
