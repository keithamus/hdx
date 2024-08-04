use hdx_proc_macro::*;

use crate::css::units::LengthPercentage;

/**
 * css-position
 *
 */

/*
 * https://drafts.csswg.org/css-position/#position-property
 * 2. Choosing A Positioning Scheme: position property
 */
// https://drafts.csswg.org/css-position/#propdef-position
#[value(" static | relative | absolute | sticky | fixed ")]
#[initial("static")]
#[applies_to("all elements except table-column-group and table-column")]
#[inherited("no")]
#[canonical_order("per grammar")]
pub enum Position {}

/*
 * 3. Positioning Coordinates
 */

/*
 * https://drafts.csswg.org/css-position/#insets
 * 3.1. Box Insets: the top, right, bottom, left, inset-block-start, inset-inline-start, inset-block-end, and inset-inline-end properties
 */

// https://drafts.csswg.org/css-position/#propdef-top
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum Top {}

// https://drafts.csswg.org/css-position/#propdef-right
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum Right {}

// https://drafts.csswg.org/css-position/#propdef-bottom
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum Bottom {}

// https://drafts.csswg.org/css-position/#propdef-left
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum Left {}

// https://drafts.csswg.org/css-position/#propdef-inset-block-start
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum InsetBlockStart {}

// https://drafts.csswg.org/css-position/#propdef-inset-inline-start
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum InsetInlineStart {}

// https://drafts.csswg.org/css-position/#propdef-inset-block-end
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub enum InsetBlockEnd {}

// https://drafts.csswg.org/css-position/#propdef-inset-block
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub struct InsetBlock;

// https://drafts.csswg.org/css-position/#propdef-inset-inline
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub struct InsetInline;

// https://drafts.csswg.org/css-position/#propdef-inset
#[value(" <'top'>{1,4} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
pub struct InsetInline;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Position, 8);

		assert_size!(Top, 8);
		assert_size!(Right, 8);
		assert_size!(Bottom, 8);
		assert_size!(Left, 8);
		assert_size!(InsetBlockStart, 8);
		assert_size!(InsetInlineStart, 8);
		assert_size!(InsetBlockEnd, 8);
		assert_size!(InsetInlineEnd, 8);
		assert_size!(InsetBlock, 8);
		assert_size!(InsetInline, 8);
		assert_size!(Inset, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Top, "-10px");
		assert_parse!(Top, "auto");
	}

	#[test]
	fn test_writes() {
		assert_parse!(Top, "-10px");
		assert_parse!(Top, "auto");
	}
	#[test]
	fn test_errors() {
		assert_parse_error!(Top, "");
		assert_parse_error!(Top, "none");
		assert_parse_error!(Top, "30deg");
	}
}
