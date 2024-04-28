use hdx_derive::{Parsable, Value, Writable};

use crate::{css::types::Color, macros::*};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderTopColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBottomColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderLeftColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderRightColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockStartColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockEndColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineStartColor(pub Color);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineEndColor(pub Color);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-block-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockColor(pub BorderBlockStartColor, pub BorderBlockEndColor);

parse_logical_sides!(BorderBlockColor, Color, BorderBlockStartColor, BorderBlockEndColor);
write_logical_sides!(BorderBlockColor);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-inline-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineColor(pub BorderInlineStartColor, pub BorderInlineEndColor);

parse_logical_sides!(BorderInlineColor, Color, BorderInlineStartColor, BorderInlineEndColor);
write_logical_sides!(BorderInlineColor);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderColor(
	pub BorderBlockStartColor,
	pub BorderInlineEndColor,
	pub BorderBlockEndColor,
	pub BorderInlineStartColor,
);

parse_rect!(
	BorderColor,
	Color,
	BorderBlockStartColor,
	BorderInlineEndColor,
	BorderBlockEndColor,
	BorderInlineStartColor
);
write_rect!(BorderColor);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderColor, 144);
		assert_size!(BorderBlockColor, 72);
		assert_size!(BorderInlineColor, 72);
		assert_size!(BorderBlockStartColor, 36);
		assert_size!(BorderBlockEndColor, 36);
		assert_size!(BorderInlineStartColor, 36);
		assert_size!(BorderInlineEndColor, 36);
		assert_size!(BorderTopColor, 36);
		assert_size!(BorderRightColor, 36);
		assert_size!(BorderLeftColor, 36);
		assert_size!(BorderBottomColor, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderLeftColor, "red");
		assert_parse!(BorderBlockColor, "red red");
		assert_parse!(BorderBlockColor, "red red");
		assert_parse!(BorderInlineColor, "red red");
		assert_parse!(BorderInlineColor, "red red");
		assert_parse!(BorderColor, "red red red red");
		assert_parse!(BorderColor, "red red red red");
		assert_parse!(BorderColor, "red black blue green");
		assert_parse!(BorderColor, "red black blue green");
	}

	#[test]
	fn test_minify() {
		assert_minify!(BorderColor, "red red", "red");
		assert_minify!(BorderColor, "red red red", "red");
		assert_minify!(BorderColor, "red red red red", "red");
		assert_minify!(BorderColor, "red blue red blue", "red blue");
		assert_minify!(BorderColor, "red blue red", "red blue");
		assert_minify!(BorderColor, "red blue pink blue", "red blue pink");
	}
}
