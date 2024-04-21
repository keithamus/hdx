use crate::{css::types::Color, Parsable, Value, Writable};

// https://drafts.csswg.org/css-backgrounds/#background-color
#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BackgroundColor(pub Color);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BackgroundColor, 36);
	}
}
