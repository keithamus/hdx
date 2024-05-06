use hdx_derive::{Parsable, Value, Writable};

use crate::{css::units::LengthPercentageOrAuto, macros::*};

// https://drafts.csswg.org/css-box-4/#margin-physical
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Margin(pub MarginBlockStart, pub MarginBlockEnd, pub MarginInlineStart, pub MarginInlineEnd);

parse_rect!(Margin, LengthPercentageOrAuto, MarginBlockStart, MarginBlockEnd, MarginInlineStart, MarginInlineEnd);
write_rect!(Margin);

// https://drafts.csswg.org/css-logical-1/#propdef-margin-inline
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginInline(pub MarginInlineStart, pub MarginInlineEnd);

parse_logical_sides!(MarginInline, LengthPercentageOrAuto, MarginInlineStart, MarginInlineEnd);
write_logical_sides!(MarginInline);

// https://drafts.csswg.org/css-logical-1/#propdef-margin-inline
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginBlock(pub MarginBlockStart, pub MarginBlockEnd);

parse_logical_sides!(MarginBlock, LengthPercentageOrAuto, MarginBlockStart, MarginBlockEnd);
write_logical_sides!(MarginBlock);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginBlockStart(pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginBlockEnd(pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginInlineStart(pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginInlineEnd(pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginTop(LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginBottom(LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginLeft(LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MarginRight(LengthPercentageOrAuto);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Margin, 32);
		assert_size!(MarginBlockStart, 8);
		assert_size!(MarginBlockEnd, 8);
		assert_size!(MarginInlineStart, 8);
		assert_size!(MarginInlineEnd, 8);
		assert_size!(MarginTop, 8);
		assert_size!(MarginRight, 8);
		assert_size!(MarginLeft, 8);
		assert_size!(MarginBottom, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MarginLeft, "auto");
		assert_parse!(MarginBlock, "1px 1px");
		assert_parse!(MarginBlock, "1px 2px");
		assert_parse!(MarginInline, "1px 1px");
		assert_parse!(MarginInline, "1px 2px");
		assert_parse!(Margin, "1px 2px 3px 4px");
	}

	#[test]
	fn test_minify() {
		assert_minify!(MarginBlock, "1px 1px", "1px");
		assert_minify!(MarginInline, "1px 1px", "1px");
		assert_minify!(Margin, "1px", "1px");
		assert_minify!(Margin, "1px 2px 1px 2px", "1px 2px");
		assert_minify!(Margin, "1px 2px 3px", "1px 2px 3px");
		assert_minify!(Margin, "1px 2px 3px 2px", "1px 2px 3px");
	}
}
