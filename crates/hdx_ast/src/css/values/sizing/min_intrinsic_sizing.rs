use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-sizing-4/#propdef-min-intrinsic-sizing
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MinIntrinsicSizing {
	#[default]
	Legacy, // atom!("legacy")
	ZeroIfScroll,    // atom!("zero-if-scroll")
	ZeroIfExtrinsic, // atom!("zero-if-extrinsic")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MinIntrinsicSizing, 1);
	}
}
