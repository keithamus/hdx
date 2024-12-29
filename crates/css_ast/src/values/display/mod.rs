mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-display-4/
 * CSS Display Module Level 4
 */

// // https://drafts.csswg.org/css-display-4/#display
// #[value(" [ <display-outside> || <display-inside> ] | <display-listitem> | <display-internal> | <display-box> | <display-legacy> ")]
// #[initial("inline")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see § 2.9 animating and interpolating display")]
// pub enum DisplayStyleValue {}

// https://drafts.csswg.org/css-display-4/#order
#[value(" <integer> ")]
#[initial("0")]
#[applies_to("flex items and grid items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct OrderStyleValue;

// https://drafts.csswg.org/css-display-4/#visibility
#[value(" visible | hidden | collapse ")]
#[initial("visible")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum VisibilityStyleValue {}

// https://drafts.csswg.org/css-display-4/#reading-flow
#[value(" normal | flex-visual | flex-flow | grid-rows | grid-columns | grid-order ")]
#[initial("normal")]
#[applies_to("flex and grid containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum ReadingFlowStyleValue {}
