use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::{CSSFloat, Percent};

// https://drafts.csswg.org/css-color/#transparency
#[derive(Value, Parsable, Writable, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum Opacity {
	#[parsable(Number, Check::Range(0.0..=1.0))]
	Number(CSSFloat),
	#[parsable(Dimension, Check::Range(0.0..=100.0), atom = "%")]
	Percentage(Percent),
}

impl Default for Opacity {
	fn default() -> Self {
		Self::Number(1.0.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Opacity, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Opacity, "0");
		assert_parse!(Opacity, "1");
		assert_parse!(Opacity, "100%");
		assert_parse!(Opacity, "0.9999");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(Opacity, "1", {
			"node": {"type": "number", "value": 1.0},
			"start": 0,
			"end": 1
		});
	}
}
