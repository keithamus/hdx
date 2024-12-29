mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-gcpm-4/
 * CSS Generated Content for Paged Media Module Level 4
 */

// // https://drafts.csswg.org/css-gcpm-4/#string-set
// #[value(" [ <custom-ident> <content-list> ]# | none ")]
// #[initial("none")]
// #[applies_to("all elements, but not pseudo-elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum StringSetStyleValue<'a> {}

// https://drafts.csswg.org/css-gcpm-4/#running
#[value(" <custom-ident> ")]
#[initial("none")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct RunningStyleValue;

// https://drafts.csswg.org/css-gcpm-4/#footnote-display
#[value(" block | inline | compact ")]
#[initial("block")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FootnoteDisplayStyleValue {}

// https://drafts.csswg.org/css-gcpm-4/#footnote-policy
#[value(" auto | line | block ")]
#[initial("auto")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FootnotePolicyStyleValue {}

// // https://drafts.csswg.org/css-gcpm-4/#copy-into
// #[value(" none | [ [ <custom-ident> <content-level>] [, <custom-ident> <content-level>]* ]? ")]
// #[initial("none")]
// #[applies_to("all elements and pseudo-elements, but not ::first-line or ::first-letter.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum CopyIntoStyleValue {}
