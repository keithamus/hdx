mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-scrollbars-1/
 * CSS Scrollbars Styling Module Level 1
 */

// // https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color
// #[value(" auto | <color>{2} ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum ScrollbarColor {}

// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-width
#[value(" auto | thin | none ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ScrollbarWidth {}
