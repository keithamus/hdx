mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-exclusions-1/
 * CSS Exclusions Module Level 1
 */

// https://drafts.csswg.org/css-exclusions-1/#wrap-flow
#[value(" auto | both | start | end | minimum | maximum | clear ")]
#[initial("auto")]
#[applies_to("block-level elements.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum WrapFlow {}

// https://drafts.csswg.org/css-exclusions-1/#wrap-through
#[value(" wrap | none ")]
#[initial("wrap")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum WrapThrough {}
