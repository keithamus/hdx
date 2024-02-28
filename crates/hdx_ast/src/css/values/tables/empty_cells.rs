#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-tables-3/#propdef-empty-cells
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum EmptyCells {
	#[default]
	Show, // atom!("show")
	Hide, // atom!("hide")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<EmptyCells>(), 1);
	}
}
