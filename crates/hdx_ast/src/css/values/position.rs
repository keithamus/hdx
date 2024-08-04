use hdx_proc_macro::*;

use crate::css::units::LengthPercentage;

/**
 * https://drafts.csswg.org/css-position-3/
 * css-position 3
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
#[animation_type("discrete")]
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
#[animation_type("by computed value type")]
pub enum Top {}

// https://drafts.csswg.org/css-position/#propdef-right
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
#[animation_type("by computed value type")]
pub enum Right {}

// https://drafts.csswg.org/css-position/#propdef-bottom
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
#[animation_type("by computed value type")]
pub enum Bottom {}

// https://drafts.csswg.org/css-position/#propdef-left
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
#[animation_type("by computed value type")]
pub enum Left {}

// https://drafts.csswg.org/css-position/#propdef-inset-block-start
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
#[animation_type("by computed value type")]
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
#[animation_type("by computed value type")]
pub enum InsetBlockEnd {}

// https://drafts.csswg.org/css-position/#propdef-inset-inline-end
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("inset")]
#[animation_type("by computed value type")]
pub enum InsetInlineEnd {}

// https://drafts.csswg.org/css-position/#propdef-inset-block
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InsetBlock;

// https://drafts.csswg.org/css-position/#propdef-inset-inline
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InsetInline;

// https://drafts.csswg.org/css-position/#propdef-inset
#[value(" <'top'>{1,4} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct Inset;

/**
 * https://drafts.csswg.org/css-position-4/
 * css-position 4
 */

/*
 * https://drafts.csswg.org/css-position-4/#overlay
 * 3.4. Controlling the Top Layer: the overlay property
 */
// https://drafts.csswg.org/css-position-4/#propdef-overlay
#[value(" none | auto ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("(see prose)")]
pub enum Overlay {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Position, 1);
		assert_size!(Top, 8);
		assert_size!(Right, 8);
		assert_size!(Bottom, 8);
		assert_size!(Left, 8);
		assert_size!(InsetBlockStart, 8);
		assert_size!(InsetInlineStart, 8);
		assert_size!(InsetBlockEnd, 8);
		assert_size!(InsetInlineEnd, 8);
		assert_size!(InsetBlock, 24);
		assert_size!(InsetInline, 24);
		assert_size!(Inset, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Top, "-10px");
		assert_parse!(Top, "auto");
		// assert_parse!(Inset, "auto");
		assert_parse!(Inset, "20px 20px 20px 20px");
	}

	// #[test]
	// fn test_minify() {
	// 	assert_minify!(Inset, "1px 1px", "1px");
	// 	assert_minify!(Inset, "1px 1px 1px", "1px");
	// 	assert_minify!(Inset, "1px 1px 1px 1px", "1px");
	// }

	#[test]
	fn test_errors() {
		assert_parse_error!(Top, "");
		assert_parse_error!(Top, "none");
		assert_parse_error!(Top, "30deg");
	}
}
