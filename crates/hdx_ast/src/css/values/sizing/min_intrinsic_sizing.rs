#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-sizing-4/#propdef-min-intrinsic-sizing
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MinIntrinsicSizing {
	#[default]
	Legacy, // atom!("legacy")
	ZeroIfScroll,    // atom!("zero-if-scroll")
	ZeroIfExtrinsic, // atom!("zero-if-extrinsic")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<MinIntrinsicSizing>(), 1);
	}
}
