mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-transforms-2/
 * CSS Transforms Module Level 2
 */

// // https://drafts.csswg.org/css-transforms-2/#transform
// #[value(" none | <transform-list> ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("refer to the size of reference box")]
// #[canonical_order("per grammar")]
// #[animation_type("transform list, see interpolation rules")]
// pub enum TransformStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#transform-origin
// #[value(" [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [[ center | left | right ] && [ center | top | bottom ]] <length>? ")]
// #[initial("50% 50%")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("refer to the size of reference box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum TransformOriginStyleValue {}

// https://drafts.csswg.org/css-transforms-2/#transform-box
#[value(" content-box | border-box | fill-box | stroke-box | view-box ")]
#[initial("view-box")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TransformBoxStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#translate
// #[value(" none | <length-percentage> [ <length-percentage> <length>? ]? ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("relative to the width of the reference box (for the first value) or the height (for the second value)")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value, but see below for none")]
// pub enum TranslateStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#rotate
// #[value(" none | <angle> | [ x | y | z | <number>{3} ] && <angle> ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as slerp, but see below for none")]
// pub enum RotateStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#scale
// #[value(" none | [ <number> | <percentage> ]{1,3} ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value, but see below for none")]
// pub enum ScaleStyleValue {}

// https://drafts.csswg.org/css-transforms-2/#transform-style
#[value(" flat | preserve-3d ")]
#[initial("flat")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TransformStyleStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#perspective
// #[value(" none | <length [0,∞]> ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum PerspectiveStyleValue {}

// // https://drafts.csswg.org/css-transforms-2/#perspective-origin
// #[value(" <position> ")]
// #[initial("50% 50%")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("refer to the size of the reference box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct PerspectiveOriginStyleValue;

// https://drafts.csswg.org/css-transforms-2/#backface-visibility
#[value(" visible | hidden ")]
#[initial("visible")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BackfaceVisibilityStyleValue {}
