mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-line-grid-1/
 * CSS Line Grid Module Level 1
 */

// https://drafts.csswg.org/css-line-grid-1/#line-grid
#[value(" match-parent | create ")]
#[initial("match-parent")]
#[applies_to("block, flex and grid containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum LineGridStyleValue {}

// https://drafts.csswg.org/css-line-grid-1/#line-snap
#[value(" none | baseline | contain ")]
#[initial("none")]
#[applies_to("block container elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum LineSnapStyleValue {}

// https://drafts.csswg.org/css-line-grid-1/#box-snap
#[value(" none | block-start | block-end | center | baseline | last-baseline ")]
#[initial("none")]
#[applies_to("block-level boxes and internal table elements except table cells")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BoxSnapStyleValue {}
