mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-writing-modes-4/
 * CSS Writing Modes Level 4
 */

// https://drafts.csswg.org/css-writing-modes-4/#direction
#[value(" ltr | rtl ")]
#[initial("ltr")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
pub enum DirectionStyleValue {}

// https://drafts.csswg.org/css-writing-modes-4/#unicode-bidi
#[value(" normal | embed | isolate | bidi-override | isolate-override | plaintext ")]
#[initial("normal")]
#[applies_to("all elements, but see prose")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum UnicodeBidiStyleValue {}

// https://drafts.csswg.org/css-writing-modes-4/#writing-mode
#[value(" horizontal-tb | vertical-rl | vertical-lr | sideways-rl | sideways-lr ")]
#[initial("horizontal-tb")]
#[applies_to("All elements except table row groups, table column groups, table rows, table columns, ruby base containers, ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
pub enum WritingModeStyleValue {}

// https://drafts.csswg.org/css-writing-modes-4/#text-orientation
#[value(" mixed | upright | sideways ")]
#[initial("mixed")]
#[applies_to("all elements except table row groups, rows, column groups, and columns")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
pub enum TextOrientationStyleValue {}

// // https://drafts.csswg.org/css-writing-modes-4/#glyph-orientation-vertical
// #[value(" auto | 0deg | 90deg | 0 | 90 ")]
// #[initial("n/a")]
// #[applies_to("n/a")]
// #[inherited("n/a")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("n/a")]
// pub enum GlyphOrientationVerticalStyleValue {}

// // https://drafts.csswg.org/css-writing-modes-4/#text-combine-upright
// #[value(" none | all | [ digits <integer [2,4]>? ] ")]
// #[initial("none")]
// #[applies_to("inline boxes and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("not animatable")]
// pub enum TextCombineUprightStyleValue {}
