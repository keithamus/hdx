mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-values-5/
 * CSS Values and Units Module Level 5
 */

// https://drafts.csswg.org/css-values-5/#interpolate-size
#[value(" numeric-only | allow-keywords ")]
#[initial("numeric-only")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum InterpolateSizeStyleValue {}
