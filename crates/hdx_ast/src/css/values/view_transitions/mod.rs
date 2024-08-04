mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-view-transitions-2/
 * CSS View Transitions Module Level 2
 */

// https://drafts.csswg.org/css-view-transitions-2/#view-transition-name
#[value(" none | <custom-ident> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ViewTransitionName {}

// // https://drafts.csswg.org/css-view-transitions-2/#view-transition-class
// #[value(" none | <custom-ident>+ ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ViewTransitionClass {}

// https://drafts.csswg.org/css-view-transitions-2/#view-transition-group
#[value(" normal | contain | nearest | <custom-ident> ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ViewTransitionGroup {}
