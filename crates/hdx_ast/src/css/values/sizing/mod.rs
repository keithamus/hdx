mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-sizing-4/
 * CSS Box Sizing Module Level 4
 */

// https://drafts.csswg.org/css-sizing-4/#width
#[value(" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("auto")]
#[applies_to("all elements except non-replaced inlines")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type, recursing into fit-content()")]
pub enum WidthStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#height
#[value(" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("auto")]
#[applies_to("all elements except non-replaced inlines")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type, recursing into fit-content()")]
pub enum HeightStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#min-width
#[value(" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("auto")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
pub enum MinWidthStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#min-height
#[value(" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("auto")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
pub enum MinHeightStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#max-width
#[value(" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("none")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
pub enum MaxWidthStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#max-height
#[value(" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content ")]
#[initial("none")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
pub enum MaxHeightStyleValue {}

// https://drafts.csswg.org/css-sizing-4/#box-sizing
#[value(" content-box | border-box ")]
#[initial("content-box")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BoxSizingStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#aspect-ratio
// #[value(" auto || <ratio> ")]
// #[initial("auto")]
// #[applies_to("all elements except inline boxes and internal ruby or table boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct AspectRatioStyleValue;

// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-width
// #[value(" auto? [ none | <length> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum ContainIntrinsicWidthStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-height
// #[value(" auto? [ none | <length> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum ContainIntrinsicHeightStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-block-size
// #[value(" auto? [ none | <length> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum ContainIntrinsicBlockSizeStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-inline-size
// #[value(" auto? [ none | <length> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum ContainIntrinsicInlineSizeStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-size
// #[value(" [ auto? [ none | <length> ] ]{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum ContainIntrinsicSizeStyleValue {}

// // https://drafts.csswg.org/css-sizing-4/#min-intrinsic-sizing
// #[value(" legacy | zero-if-scroll || zero-if-extrinsic ")]
// #[initial("legacy")]
// #[applies_to("all elements except inline boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum MinIntrinsicSizingStyleValue {}
