use hdx_parser::{Parse, Parser, Spanned, Result as ParserResult, FromToken, unexpected};
use hdx_writer::{WriteCss, CssWriter, Result as WriterResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::LengthPercentageOrAuto, Parsable, Value, Writable, macros::*};

// https://drafts.csswg.org/css-box-4/#padding-physical
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Padding(pub PaddingBlockStart, pub PaddingBlockEnd, pub PaddingInlineStart, pub PaddingInlineEnd);

parse_rect!(Padding, LengthPercentageOrAuto, PaddingBlockStart, PaddingBlockEnd, PaddingInlineStart, PaddingInlineEnd);
write_rect!(Padding);

// https://drafts.csswg.org/css-logical-1/#propdef-padding-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingInline(pub PaddingInlineStart, pub PaddingInlineEnd);

parse_logical_sides!(PaddingInline, LengthPercentageOrAuto, PaddingInlineStart, PaddingInlineEnd);
write_logical_sides!(PaddingInline);

// https://drafts.csswg.org/css-logical-1/#propdef-padding-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingBlock(pub PaddingBlockStart, pub PaddingBlockEnd);

parse_logical_sides!(PaddingBlock, LengthPercentageOrAuto, PaddingBlockStart, PaddingBlockEnd);
write_logical_sides!(PaddingBlock);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingBlockStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingBlockEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingInlineStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingInlineEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingTop(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingBottom(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingLeft(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct PaddingRight(#[parsable(FromToken)] LengthPercentageOrAuto);

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Padding>(), 32);
		assert_eq!(size_of::<PaddingBlockStart>(), 8);
		assert_eq!(size_of::<PaddingBlockEnd>(), 8);
		assert_eq!(size_of::<PaddingInlineStart>(), 8);
		assert_eq!(size_of::<PaddingInlineEnd>(), 8);
		assert_eq!(size_of::<PaddingTop>(), 8);
		assert_eq!(size_of::<PaddingRight>(), 8);
		assert_eq!(size_of::<PaddingLeft>(), 8);
		assert_eq!(size_of::<PaddingBottom>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<PaddingLeft>(&allocator, "auto", "auto");
		test_write::<PaddingBlock>(&allocator, "1px", "1px");
		test_write::<PaddingBlock>(&allocator, "1px 2px", "1px 2px");
		test_write::<PaddingInline>(&allocator, "1px", "1px");
		test_write::<PaddingInline>(&allocator, "1px 2px", "1px 2px");
		test_write::<Padding>(&allocator, "1px", "1px");
		test_write::<Padding>(&allocator, "1px 2px", "1px 2px");
		test_write::<Padding>(&allocator, "1px 2px 3px", "1px 2px 3px");
		test_write::<Padding>(&allocator, "1px 2px 3px 4px", "1px 2px 3px 4px");
	}
}
