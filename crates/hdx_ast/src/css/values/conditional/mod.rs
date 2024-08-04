mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-conditional-5/
 * CSS Conditional Rules Module Level 5
 */

// // https://drafts.csswg.org/css-conditional-5/#container-type
// #[value(" normal | [ [ size | inline-size ] || scroll-state ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum ContainerType {}

// // https://drafts.csswg.org/css-conditional-5/#container-name
// #[value(" none | <custom-ident>+ ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum ContainerName {}

// // https://drafts.csswg.org/css-conditional-5/#container
// #[value(" <'container-name'> [ / <'container-type'> ]? ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct Container;
