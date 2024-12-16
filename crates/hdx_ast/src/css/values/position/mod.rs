mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-position-4/
 * CSS Positioned Layout Module Level 4
 */

// https://drafts.csswg.org/css-position-4/#position
#[value(" static | relative | absolute | sticky | fixed ")]
#[initial("static")]
#[applies_to("all elements except table-column-group and table-column")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum PositionStyleValue {}

// https://drafts.csswg.org/css-position-4/#top
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum TopStyleValue {}

// https://drafts.csswg.org/css-position-4/#right
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RightStyleValue {}

// https://drafts.csswg.org/css-position-4/#bottom
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum BottomStyleValue {}

// https://drafts.csswg.org/css-position-4/#left
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum LeftStyleValue {}

// https://drafts.csswg.org/css-position-4/#inset-block-start
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InsetBlockStartStyleValue {}

// https://drafts.csswg.org/css-position-4/#inset-inline-start
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InsetInlineStartStyleValue {}

// https://drafts.csswg.org/css-position-4/#inset-block-end
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InsetBlockEndStyleValue {}

// https://drafts.csswg.org/css-position-4/#inset-inline-end
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InsetInlineEndStyleValue {}

// https://drafts.csswg.org/css-position-4/#inset-block
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InsetBlockStyleValue;

// https://drafts.csswg.org/css-position-4/#inset-inline
#[value(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InsetInlineStyleValue;

// https://drafts.csswg.org/css-position-4/#inset
#[value(" <'top'>{1,4} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InsetStyleValue;

// https://drafts.csswg.org/css-position-4/#overlay
#[value(" none | auto ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum OverlayStyleValue {}
