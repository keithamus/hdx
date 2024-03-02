#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentageOrAuto, macros::*, Parsable, Value, Writable};

// https://drafts.csswg.org/css-box-4/#margin-physical
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Margin(pub MarginBlockStart, pub MarginBlockEnd, pub MarginInlineStart, pub MarginInlineEnd);

parse_rect!(Margin, LengthPercentageOrAuto, MarginBlockStart, MarginBlockEnd, MarginInlineStart, MarginInlineEnd);
write_rect!(Margin);

// https://drafts.csswg.org/css-logical-1/#propdef-margin-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginInline(pub MarginInlineStart, pub MarginInlineEnd);

parse_logical_sides!(MarginInline, LengthPercentageOrAuto, MarginInlineStart, MarginInlineEnd);
write_logical_sides!(MarginInline);

// https://drafts.csswg.org/css-logical-1/#propdef-margin-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginBlock(pub MarginBlockStart, pub MarginBlockEnd);

parse_logical_sides!(MarginBlock, LengthPercentageOrAuto, MarginBlockStart, MarginBlockEnd);
write_logical_sides!(MarginBlock);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginBlockStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginBlockEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginInlineStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginInlineEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginTop(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginBottom(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginLeft(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginRight(#[parsable(FromToken)] LengthPercentageOrAuto);

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
		assert_parse!(MarginBlock, "1px");
		assert_parse!(MarginBlock, "1px 2px");
		assert_parse!(MarginInline, "1px");
		assert_parse!(MarginInline, "1px 2px");
		assert_parse!(Margin, "1px");
		assert_parse!(Margin, "1px 2px");
		assert_parse!(Margin, "1px 2px 3px");
		assert_parse!(Margin, "1px 2px 3px 4px");
	}
}
