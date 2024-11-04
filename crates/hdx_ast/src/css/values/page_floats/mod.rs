mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-page-floats-3/
 * CSS Page Floats
 */

// https://drafts.csswg.org/css-page-floats-3/#float-reference
#[value(" inline | column | region | page ")]
#[initial("inline")]
#[applies_to("all elements.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FloatReference {}

// // https://drafts.csswg.org/css-page-floats-3/#float
// #[value(" block-start | block-end | inline-start | inline-end | snap-block | <snap-block()> | snap-inline | <snap-inline()> | left | right | top | bottom | none ")]
// #[initial("none")]
// #[applies_to("all elements.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum Float {}

// https://drafts.csswg.org/css-page-floats-3/#clear
#[value(" inline-start | inline-end | block-start | block-end | left | right | top | bottom | both-inline | both-block | both | none ")]
#[initial("none")]
#[applies_to("block-level elements, floats, regions, pages")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum Clear {}

// https://drafts.csswg.org/css-page-floats-3/#float-defer
#[value(" <integer> | last | none ")]
#[initial("none")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FloatDefer {}

// https://drafts.csswg.org/css-page-floats-3/#float-offset
#[value(" <length-percentage> ")]
#[initial("0")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct FloatOffset;
