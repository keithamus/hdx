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
pub enum BlockStepSizeStyleValue {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-insert
#[value(" margin-box | padding-box | content-box ")]
#[initial("margin-box")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepInsertStyleValue {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-align
#[value(" auto | center | start | end ")]
#[initial("auto")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepAlignStyleValue {}

// https://drafts.csswg.org/css-rhythm-1/#block-step-round
#[value(" up | down | nearest ")]
#[initial("up")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockStepRoundStyleValue {}

// https://drafts.csswg.org/css-rhythm-1/#block-step
#[value(" <'block-step-size'> || <'block-step-insert'> || <'block-step-align'> || <'block-step-round'> ")]
#[initial("see individual properties")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BlockStepStyleValue;

// https://drafts.csswg.org/css-rhythm-1/#line-height-step
#[value(" <length [0,∞]> ")]
#[initial("0")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct LineHeightStepStyleValue;
