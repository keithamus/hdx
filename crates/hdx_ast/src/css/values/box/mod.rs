mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-box-4/
 * CSS Box Model Module Level 4
 */

// https://drafts.csswg.org/css-box-4/#margin-top
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MarginTopStyleValue {}

// https://drafts.csswg.org/css-box-4/#margin-right
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MarginRightStyleValue {}

// https://drafts.csswg.org/css-box-4/#margin-bottom
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MarginBottomStyleValue {}

// https://drafts.csswg.org/css-box-4/#margin-left
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MarginLeftStyleValue {}

// https://drafts.csswg.org/css-box-4/#margin
#[value(" <'margin-top'>{1,4} ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MarginStyleValue;

// https://drafts.csswg.org/css-box-4/#padding-top
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingTopStyleValue;

// https://drafts.csswg.org/css-box-4/#padding-right
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingRightStyleValue;

// https://drafts.csswg.org/css-box-4/#padding-bottom
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingBottomStyleValue;

// https://drafts.csswg.org/css-box-4/#padding-left
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingLeftStyleValue;

// https://drafts.csswg.org/css-box-4/#padding
#[value(" <'padding-top'>{1,4} ")]
#[initial("0")]
#[applies_to("all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingStyleValue;

// // https://drafts.csswg.org/css-box-4/#margin-trim
// #[value(" none | [ block || inline ] | [ block-start || inline-start || block-end || inline-end ] ")]
// #[initial("none")]
// #[applies_to("block containers, multi-column containers, flex containers, grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum MarginTrimStyleValue {}
