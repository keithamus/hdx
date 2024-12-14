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
// pub enum AnimationNameStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-duration
// #[value(" [ auto | <time [0s,∞]> ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationDurationStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-timing-function
// #[value(" <easing-function># ")]
// #[initial("ease")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationTimingFunctionStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[value(" <single-animation-iteration-count># ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationIterationCountStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-direction
#[value(" <single-animation-direction># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDirectionStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[value(" <single-animation-play-state># ")]
#[initial("running")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationPlayStateStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-delay
#[value(" <time># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDelayStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[value(" <single-animation-fill-mode># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationFillModeStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation
// #[value(" <single-animation># ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-composition
#[value(" <single-animation-composition># ")]
#[initial("replace")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationCompositionStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation-timeline
// #[value(" <single-animation-timeline># ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationTimelineStyleValue<'a>;
