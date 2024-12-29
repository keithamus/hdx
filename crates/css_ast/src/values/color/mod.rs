mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-color-6/
 * CSS Color Module Level 4
 */

// https://drafts.csswg.org/css-color-6/#color
#[value(" <color> ")]
#[initial("CanvasText")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ColorStyleValue;

// https://drafts.csswg.org/css-color-6/#opacity
#[value(" <opacity-value> ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("map to the range [0,1]")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct OpacityStyleValue;
