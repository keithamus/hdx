mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-scroll-anchoring-1/
 * CSS Scroll Anchoring Module Level 1
 */

// https://drafts.csswg.org/css-scroll-anchoring-1/#overflow-anchor
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverflowAnchor {}
