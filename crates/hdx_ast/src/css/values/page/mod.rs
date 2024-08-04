mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-page-4/
 * CSS Paged Media Module Level 3
 */

// https://drafts.csswg.org/css-page-4/#page
#[value(" auto | <custom-ident> ")]
#[initial("auto")]
#[applies_to("boxes that create class A break points")]
#[inherited("no (but see prose)")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum Page {}
