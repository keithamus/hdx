mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-contain-4/
 * CSS Containment Module Level 2
 */

// // https://drafts.csswg.org/css-contain-4/#contain
// #[value(" none | strict | content | [ [size | inline-size] || layout || style || paint ] ")]
// #[initial("none")]
// #[applies_to("See below")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum Contain {}

// https://drafts.csswg.org/css-contain-4/#content-visibility
#[value(" visible | auto | hidden ")]
#[initial("visible")]
#[applies_to("elements for which size containment can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see § 4.1 animating and interpolating content-visibility")]
pub enum ContentVisibility {}
