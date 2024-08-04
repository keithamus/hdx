mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-shapes-2/
 * CSS Shapes Module Level 2
 */

// // https://drafts.csswg.org/css-shapes-2/#shape-outside
// #[value(" none | [ <basic-shape> || <shape-box> ] | <image> ")]
// #[initial("none")]
// #[applies_to("floats and initial letter boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as defined for <basic-shape>, otherwise discrete")]
// pub enum ShapeOutside {}

// https://drafts.csswg.org/css-shapes-2/#shape-image-threshold
#[value(" <opacity-value> ")]
#[initial("0")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct ShapeImageThreshold;

// https://drafts.csswg.org/css-shapes-2/#shape-margin
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("floats and initial letter boxes")]
#[inherited("no")]
#[percentages("refer to the inline size of the containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct ShapeMargin;

// // https://drafts.csswg.org/css-shapes-2/#shape-inside
// #[value(" auto | outside-shape | [ <basic-shape> || shape-box ] | <image> | display ")]
// #[initial("auto")]
// #[applies_to("block-level elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as defined for <basic-shape>, otherwise discrete")]
// pub enum ShapeInside {}

// https://drafts.csswg.org/css-shapes-2/#shape-padding
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("refer to the inline size of the containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct ShapePadding;
