mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-animations-2/
 * CSS Animations Level 2
 */

// // https://drafts.csswg.org/css-animations-2/#animation-name
// #[value(" [ none | <keyframes-name> ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationName {}

// // https://drafts.csswg.org/css-animations-2/#animation-duration
// #[value(" [ auto | <time [0s,∞]> ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationDuration {}

// // https://drafts.csswg.org/css-animations-2/#animation-timing-function
// #[value(" <easing-function># ")]
// #[initial("ease")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationTimingFunction;

// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[value(" <single-animation-iteration-count># ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationIterationCount;

// https://drafts.csswg.org/css-animations-2/#animation-direction
#[value(" <single-animation-direction># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDirection;

// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[value(" <single-animation-play-state># ")]
#[initial("running")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationPlayState;

// https://drafts.csswg.org/css-animations-2/#animation-delay
#[value(" <time># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDelay;

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[value(" <single-animation-fill-mode># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationFillMode;

// // https://drafts.csswg.org/css-animations-2/#animation
// #[value(" <single-animation># ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct Animation;

// https://drafts.csswg.org/css-animations-2/#animation-composition
#[value(" <single-animation-composition># ")]
#[initial("replace")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationComposition;

// // https://drafts.csswg.org/css-animations-2/#animation-timeline
// #[value(" <single-animation-timeline># ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationTimeline;
