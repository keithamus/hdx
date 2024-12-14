mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-content-3/
 * CSS Generated Content Module Level 3
 */

// // https://drafts.csswg.org/css-content-3/#content
// #[value(" normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]? ")]
// #[initial("normal")]
// #[applies_to("all elements, tree-abiding pseudo-elements, and page margin boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ContentStyleValue<'a> {}

// // https://drafts.csswg.org/css-content-3/#quotes
// #[value(" auto | none | match-parent | [ <string> <string> ]+ ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum QuotesStyleValue<'a> {}

// // https://drafts.csswg.org/css-content-3/#string-set
// #[value(" none | [ <custom-ident> <string>+ ]# ")]
// #[initial("none")]
// #[applies_to("all elements, but not pseudo-elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum StringSetStyleValue<'a> {}

// https://drafts.csswg.org/css-content-3/#bookmark-level
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum BookmarkLevelStyleValue {}

// // https://drafts.csswg.org/css-content-3/#bookmark-label
// #[value(" <content-list> ")]
// #[initial("content(text)")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BookmarkLabelStyleValue;

// https://drafts.csswg.org/css-content-3/#bookmark-state
#[value(" open | closed ")]
#[initial("open")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BookmarkStateStyleValue {}
