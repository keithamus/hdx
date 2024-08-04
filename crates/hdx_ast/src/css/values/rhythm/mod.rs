mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-rhythm-1/
 * CSS Rhythmic Sizing
 */

// https://drafts.csswg.org/css-rhythm-1/#block-step-size
#[value(" none | <length [0,∞]> ")]
#[initial("none")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum BlockStepSize {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-insert
#[value(" margin | padding ")]
#[initial("margin")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepInsert {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-align
#[value(" auto | center | start | end ")]
#[initial("auto")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepAlign {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-round
#[value(" up | down | nearest ")]
#[initial("up")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepRound {}

// https://drafts.csswg.org/css-rhythm-1/#block-step
#[value(" <'block-step-size'> || <'block-step-insert'> || <'block-step-align'> || <'block-step-round'> ")]
#[initial("see individual properties")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BlockStep;

// https://drafts.csswg.org/css-rhythm-1/#line-height-step
#[value(" <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct LineHeightStep;
