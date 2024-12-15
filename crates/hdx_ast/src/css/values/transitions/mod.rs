mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-transitions-2/
 * CSS Transitions Level 2
 */

// // https://drafts.csswg.org/css-transitions-2/#transition-property
// #[value(" none | <single-transition-property># ")]
// #[initial("all")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum TransitionPropertyStyleValue<'a> {}

// https://drafts.csswg.org/css-transitions-2/#transition-duration
#[value(" <time [0s,∞]># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct TransitionDurationStyleValue<'a>;

// // https://drafts.csswg.org/css-transitions-2/#transition-timing-function
// #[value(" <easing-function># ")]
// #[initial("ease")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct TransitionTimingFunctionStyleValue<'a>;

// https://drafts.csswg.org/css-transitions-2/#transition-delay
#[value(" <time># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct TransitionDelayStyleValue<'a>;

// // https://drafts.csswg.org/css-transitions-2/#transition
// #[value(" <single-transition># ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct TransitionStyleValue<'a>;

// https://drafts.csswg.org/css-transitions-2/#transition-behavior
#[value(" <transition-behavior-value># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct TransitionBehaviorStyleValue<'a>;
