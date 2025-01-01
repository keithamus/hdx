mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-cascade-6/
 * CSS Cascading and Inheritance Level 5
 */

// https://drafts.csswg.org/css-cascade-6/#all
#[value(" initial | inherit | unset | revert | revert-layer ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub enum AllStyleValue {}
