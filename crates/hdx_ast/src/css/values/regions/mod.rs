mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-regions-1/
 * CSS Regions Module Level 1
 */

// // https://drafts.csswg.org/css-regions-1/#flow-into
// #[value(" none | <ident> [element | content]? ")]
// #[initial("none")]
// #[applies_to("All elements, but not pseudo-elements such as ::first-line, ::first-letter, ::before or ::after.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum FlowIntoStyleValue {}

// // https://drafts.csswg.org/css-regions-1/#flow-from
// #[value(" <ident> | none ")]
// #[initial("none")]
// #[applies_to("Non-replaced block containers.  This might be expanded in future versions of the specification to allow other types of containers to receive flow content.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum FlowFromStyleValue {}

// https://drafts.csswg.org/css-regions-1/#region-fragment
#[value(" auto | break ")]
#[initial("auto")]
#[applies_to("CSS Regions")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum RegionFragmentStyleValue {}
