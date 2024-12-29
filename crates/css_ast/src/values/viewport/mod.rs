mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-viewport-1/
 * CSS Viewport Module Level 1
 */

// https://drafts.csswg.org/css-viewport-1/#zoom
#[value(" <number [0,∞]> || <percentage [0,∞]> ")]
#[initial("1")]
#[applies_to("all <length> property values of all elements")]
#[inherited("no")]
#[percentages("converted to <number>")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct ZoomStyleValue;
