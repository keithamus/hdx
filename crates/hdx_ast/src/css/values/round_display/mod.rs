mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-round-display-1/
 * CSS Round Display Level 1
 */

// https://drafts.csswg.org/css-round-display-1/#border-boundary
#[value(" none | parent | display ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BorderBoundary {}
