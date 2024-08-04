mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-flexbox-1/
 * CSS Flexible Box Layout Module Level 1
 */

// https://drafts.csswg.org/css-flexbox-1/#flex-direction
#[value(" row | row-reverse | column | column-reverse ")]
#[initial("row")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FlexDirection {}

// https://drafts.csswg.org/css-flexbox-1/#flex-wrap
#[value(" nowrap | wrap | wrap-reverse ")]
#[initial("nowrap")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FlexWrap {}

// https://drafts.csswg.org/css-flexbox-1/#flex-flow
#[value(" <'flex-direction'> || <'flex-wrap'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct FlexFlow;

// // https://drafts.csswg.org/css-flexbox-1/#flex
// #[value(" none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ] ")]
// #[initial("0 1 auto")]
// #[applies_to("flex items")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum Flex {}

// https://drafts.csswg.org/css-flexbox-1/#flex-grow
#[value(" <number [0,∞]> ")]
#[initial("0")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct FlexGrow;

// https://drafts.csswg.org/css-flexbox-1/#flex-shrink
#[value(" <number [0,∞]> ")]
#[initial("1")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("number")]
pub struct FlexShrink;

// // https://drafts.csswg.org/css-flexbox-1/#flex-basis
// #[value(" content | <'width'> ")]
// #[initial("auto")]
// #[applies_to("flex items")]
// #[inherited("no")]
// #[percentages("relative to the flex container’s inner main size")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum FlexBasis {}

// https://drafts.csswg.org/css-flexbox-1/#justify-content
#[value(" flex-start | flex-end | center | space-between | space-around ")]
#[initial("flex-start")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum JustifyContent {}

// https://drafts.csswg.org/css-flexbox-1/#align-items
#[value(" flex-start | flex-end | center | baseline | stretch ")]
#[initial("stretch")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AlignItems {}

// https://drafts.csswg.org/css-flexbox-1/#align-self
#[value(" auto | flex-start | flex-end | center | baseline | stretch ")]
#[initial("auto")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AlignSelf {}

// https://drafts.csswg.org/css-flexbox-1/#align-content
#[value(" flex-start | flex-end | center | space-between | space-around | stretch ")]
#[initial("stretch")]
#[applies_to("multi-line flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AlignContent {}
