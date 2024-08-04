mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-images-5/
 * CSS Images Module Level 5
 */

// // https://drafts.csswg.org/css-images-5/#object-fit
// #[value(" fill | none | [contain | cover] || scale-down ")]
// #[initial("fill")]
// #[applies_to("replaced elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ObjectFit {}

// // https://drafts.csswg.org/css-images-5/#object-position
// #[value(" <position> ")]
// #[initial("50% 50%")]
// #[applies_to("replaced elements")]
// #[inherited("no")]
// #[percentages("refer to width and height of element itself")]
// #[canonical_order("the horizontal component of the <position>, followed by the vertical component")]
// #[animation_type("as for background-position")]
// pub struct ObjectPosition;

// // https://drafts.csswg.org/css-images-5/#image-orientation
// #[value(" from-image | none | [ <angle> || flip ] ")]
// #[initial("from-image")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ImageOrientation {}

// https://drafts.csswg.org/css-images-5/#image-rendering
#[value(" auto | smooth | high-quality | pixelated | crisp-edges ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ImageRendering {}

// // https://drafts.csswg.org/css-images-5/#image-resolution
// #[value(" [ from-image || <resolution> ] && snap? ")]
// #[initial("1dppx")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct ImageResolution;

// // https://drafts.csswg.org/css-images-5/#object-view-box
// #[value(" none | <basic-shape-rect> ")]
// #[initial("none")]
// #[applies_to("replaced elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as  if possible, otherwise discrete")]
// pub enum ObjectViewBox {}
