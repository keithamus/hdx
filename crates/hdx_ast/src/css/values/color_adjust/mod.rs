mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-color-adjust-1/
 * CSS Color Adjustment Module Level 1
 */

// // https://drafts.csswg.org/css-color-adjust-1/#color-scheme
// #[value(" normal | [ light | dark | <custom-ident> ]+ && only? ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ColorScheme {}

// https://drafts.csswg.org/css-color-adjust-1/#forced-color-adjust
#[value(" auto | none | preserve-parent-color ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum ForcedColorAdjust {}

// https://drafts.csswg.org/css-color-adjust-1/#print-color-adjust
#[value(" economy | exact ")]
#[initial("economy")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum PrintColorAdjust {}

// https://drafts.csswg.org/css-color-adjust-1/#color-adjust
#[value(" <'print-color-adjust'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct ColorAdjust;
