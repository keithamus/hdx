mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-anchor-position-1/
 * CSS Anchor Positioning
 */

// https://drafts.csswg.org/css-anchor-position-1/#anchor-name
#[value(" none | <dashed-ident># ")]
#[initial("none")]
#[applies_to("all elements that generate a principal box")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AnchorNameStyleValue<'a> {}

// https://drafts.csswg.org/css-anchor-position-1/#anchor-scope
#[value(" none | all | <dashed-ident># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AnchorScopeStyleValue<'a> {}

// https://drafts.csswg.org/css-anchor-position-1/#position-anchor
#[value(" auto | <anchor-name> ")]
#[initial("auto")]
#[applies_to("absolutely positioned boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum PositionAnchorStyleValue {}

// https://drafts.csswg.org/css-anchor-position-1/#position-area
#[value(" none | <position-area> ")]
#[initial("none")]
#[applies_to("positioned boxes with a default anchor box")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("tbd")]
pub enum PositionAreaStyleValue {}

// // https://drafts.csswg.org/css-anchor-position-1/#position-visibility
// #[value(" always | [ anchors-valid || anchors-visible || no-overflow ] ")]
// #[initial("anchors-visible")]
// #[applies_to("absolutely positioned boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum PositionVisibilityStyleValue {}

// // https://drafts.csswg.org/css-anchor-position-1/#position-try-fallbacks
// #[value(" none | [ [<dashed-ident> || <try-tactic>] | <'position-area'> ]# ")]
// #[initial("none")]
// #[applies_to("absolutely positioned boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum PositionTryFallbacksStyleValue<'a> {}

// https://drafts.csswg.org/css-anchor-position-1/#position-try-order
#[value(" normal | <try-size> ")]
#[initial("normal")]
#[applies_to("absolutely positioned boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum PositionTryOrderStyleValue {}

// // https://drafts.csswg.org/css-anchor-position-1/#position-try
// #[value(" <'position-try-order'>? <'position-try-fallbacks'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct PositionTryStyleValue;
