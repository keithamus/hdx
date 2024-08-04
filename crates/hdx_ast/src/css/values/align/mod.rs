mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-align-3/
 * CSS Box Alignment Module Level 3
 */

// // https://drafts.csswg.org/css-align-3/#align-content
// #[value(" normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position> ")]
// #[initial("normal")]
// #[applies_to("block containers, multicol containers, flex containers, and grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum AlignContent {}

// // https://drafts.csswg.org/css-align-3/#justify-content
// #[value(" normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ] ")]
// #[initial("normal")]
// #[applies_to("multicol containers, flex containers, and grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum JustifyContent {}

// // https://drafts.csswg.org/css-align-3/#place-content
// #[value(" <'align-content'> <'justify-content'>? ")]
// #[initial("normal")]
// #[applies_to("block containers, flex containers, and grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct PlaceContent;

// // https://drafts.csswg.org/css-align-3/#justify-self
// #[value(" auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] ")]
// #[initial("auto")]
// #[applies_to("block-level boxes, absolutely-positioned boxes, and grid items")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum JustifySelf {}

// // https://drafts.csswg.org/css-align-3/#align-self
// #[value(" auto | normal | stretch | <baseline-position> | <overflow-position>? <self-position> ")]
// #[initial("auto")]
// #[applies_to("flex items, grid items, and absolutely-positioned boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum AlignSelf {}

// // https://drafts.csswg.org/css-align-3/#place-self
// #[value(" <'align-self'> <'justify-self'>? ")]
// #[initial("auto")]
// #[applies_to("block-level boxes, absolutely-positioned boxes, and grid items")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct PlaceSelf;

// // https://drafts.csswg.org/css-align-3/#justify-items
// #[value(" normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ] ")]
// #[initial("legacy")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum JustifyItems {}

// // https://drafts.csswg.org/css-align-3/#align-items
// #[value(" normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum AlignItems {}

// // https://drafts.csswg.org/css-align-3/#place-items
// #[value(" <'align-items'> <'justify-items'>? ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct PlaceItems;

// https://drafts.csswg.org/css-align-3/#row-gap
#[value(" normal | <length-percentage [0,∞]> ")]
#[initial("normal")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("see § 8.3 percentages in gap properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RowGap {}

// https://drafts.csswg.org/css-align-3/#column-gap
#[value(" normal | <length-percentage [0,∞]> ")]
#[initial("normal")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("see § 8.3 percentages in gap properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ColumnGap {}

// https://drafts.csswg.org/css-align-3/#gap
#[value(" <'row-gap'> <'column-gap'>? ")]
#[initial("see individual properties")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the content area")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct Gap;
