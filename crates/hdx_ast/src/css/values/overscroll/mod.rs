mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-overscroll-1/
 * CSS Overscroll Behavior Module Level 1
 */

// // https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior
// #[value(" [ contain | none | auto ]{1,2} ")]
// #[initial("auto auto")]
// #[applies_to("scroll container elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum OverscrollBehaviorStyleValue {}

// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-x
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverscrollBehaviorXStyleValue {}

// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-y
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverscrollBehaviorYStyleValue {}

// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-inline
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverscrollBehaviorInlineStyleValue {}

// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-block
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverscrollBehaviorBlockStyleValue {}
