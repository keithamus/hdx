mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-size-adjust-1/
 * CSS Mobile Text Size Adjustment Module Level 1
 */

// https://drafts.csswg.org/css-size-adjust-1/#text-size-adjust
#[value(" auto | none | <percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("see below")]
#[canonical_order("n/a")]
#[animation_type("by computed value")]
pub enum TextSizeAdjustStyleValue {}
