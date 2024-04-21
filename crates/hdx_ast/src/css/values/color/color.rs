use crate::{css::types::Color as ColorType, Parsable, Value, Writable};

// https://drafts.csswg.org/css-color/#the-color-property
#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Color(pub ColorType);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Color, 36);
	}
}
