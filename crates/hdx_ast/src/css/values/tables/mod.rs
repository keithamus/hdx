mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-tables-3/
 * CSS Table Module Level 3
 */

// https://drafts.csswg.org/css-tables-3/#table-layout
#[value(" auto | fixed ")]
#[initial("auto")]
#[applies_to("table grid boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TableLayout {}

// https://drafts.csswg.org/css-tables-3/#border-collapse
#[value(" separate | collapse ")]
#[initial("separate")]
#[applies_to("table grid boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BorderCollapse {}

// // https://drafts.csswg.org/css-tables-3/#border-spacing
// #[value(" <length>{1,2} ")]
// #[initial("0px 0px")]
// #[applies_to("table grid boxes when border-collapse is separate")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct BorderSpacing;

// https://drafts.csswg.org/css-tables-3/#caption-side
#[value(" top | bottom ")]
#[initial("top")]
#[applies_to("table-caption boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum CaptionSide {}

// https://drafts.csswg.org/css-tables-3/#empty-cells
#[value(" show | hide ")]
#[initial("show")]
#[applies_to("table-cell boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum EmptyCells {}
