use hdx_derive::{Parsable, Value, Writable};

use crate::{css::keywords::LineStyle, macros::*};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderTopStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBottomStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderLeftStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderRightStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockStartStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockEndStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineStartStyle(#[parsable(FromToken)] pub LineStyle);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineEndStyle(#[parsable(FromToken)] pub LineStyle);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-block-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBlockStyle(pub BorderBlockStartStyle, pub BorderBlockEndStyle);

parse_logical_sides!(BorderBlockStyle, LineStyle, BorderBlockStartStyle, BorderBlockEndStyle);
write_logical_sides!(BorderBlockStyle);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-inline-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderInlineStyle(pub BorderInlineStartStyle, pub BorderInlineEndStyle);

parse_logical_sides!(BorderInlineStyle, LineStyle, BorderInlineStartStyle, BorderInlineEndStyle);
write_logical_sides!(BorderInlineStyle);

// https://drafts.csswg.org/css-backgrounds/#propdef-border-width
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderStyle(
	pub BorderBlockStartStyle,
	pub BorderInlineEndStyle,
	pub BorderBlockEndStyle,
	pub BorderInlineStartStyle,
);

parse_rect!(
	BorderStyle,
	LineStyle,
	BorderBlockStartStyle,
	BorderInlineEndStyle,
	BorderBlockEndStyle,
	BorderInlineStartStyle
);
write_rect!(BorderStyle);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderStyle, 4);
		assert_size!(BorderBlockStartStyle, 1);
		assert_size!(BorderBlockEndStyle, 1);
		assert_size!(BorderInlineStartStyle, 1);
		assert_size!(BorderInlineEndStyle, 1);
		assert_size!(BorderTopStyle, 1);
		assert_size!(BorderRightStyle, 1);
		assert_size!(BorderLeftStyle, 1);
		assert_size!(BorderBottomStyle, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderLeftStyle, "solid");
		assert_parse!(BorderBlockStyle, "dashed dotted");
		assert_parse!(BorderBlockStyle, "dotted dotted");
		assert_parse!(BorderInlineStyle, "double dotted");
		assert_parse!(BorderStyle, "solid solid solid solid");
		assert_parse!(BorderStyle, "solid dashed double groove");
	}

	#[test]
	fn test_minify() {
		assert_minify!(BorderStyle, "solid solid", "solid");
		assert_minify!(BorderStyle, "solid solid solid", "solid");
		assert_minify!(BorderStyle, "solid solid solid solid", "solid");
		assert_minify!(BorderStyle, "solid double solid double", "solid double");
		assert_minify!(BorderStyle, "solid double solid", "solid double");
		assert_minify!(BorderStyle, "solid double groove double", "solid double groove");
	}
}
