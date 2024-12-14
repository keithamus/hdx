mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-lists-3/
 * CSS Lists and Counters Module Level 3
 */

// https://drafts.csswg.org/css-lists-3/#list-style-image
#[value(" <image> | none ")]
#[initial("none")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ListStyleImageStyleValue<'a> {}

// // https://drafts.csswg.org/css-lists-3/#list-style-type
// #[value(" <counter-style> | <string> | none ")]
// #[initial("disc")]
// #[applies_to("list items")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ListStyleTypeStyleValue<'a> {}

// https://drafts.csswg.org/css-lists-3/#list-style-position
#[value(" inside | outside ")]
#[initial("outside")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ListStylePositionStyleValue {}

// // https://drafts.csswg.org/css-lists-3/#list-style
// #[value(" <'list-style-position'> || <'list-style-image'> || <'list-style-type'> ")]
// #[initial("see individual properties")]
// #[applies_to("list items")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct ListStyleStyleValue;

// https://drafts.csswg.org/css-lists-3/#marker-side
#[value(" match-self | match-parent ")]
#[initial("match-self")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum MarkerSideStyleValue {}

// // https://drafts.csswg.org/css-lists-3/#counter-reset
// #[value(" [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum CounterResetStyleValue {}

// // https://drafts.csswg.org/css-lists-3/#counter-increment
// #[value(" [ <counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum CounterIncrementStyleValue {}

// // https://drafts.csswg.org/css-lists-3/#counter-set
// #[value(" [ <counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum CounterSetStyleValue {}
