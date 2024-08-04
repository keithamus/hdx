mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-nav-1/
 * CSS Spatial Navigation Level 1
 */

// https://drafts.csswg.org/css-nav-1/#spatial-navigation-contain
#[value(" auto | contain ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum SpatialNavigationContain {}

// https://drafts.csswg.org/css-nav-1/#spatial-navigation-action
#[value(" auto | focus | scroll ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum SpatialNavigationAction {}

// https://drafts.csswg.org/css-nav-1/#spatial-navigation-function
#[value(" normal | grid ")]
#[initial("normal")]
#[applies_to("spatial navigation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum SpatialNavigationFunction {}
