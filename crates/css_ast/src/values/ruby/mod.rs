mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-ruby-1/
 * CSS Ruby Annotation Layout Module Level 1
 */

// // https://drafts.csswg.org/css-ruby-1/#ruby-position
// #[value(" [ alternate || [ over | under ] ] | inter-character ")]
// #[initial("alternate")]
// #[applies_to("ruby annotation containers")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum RubyPositionStyleValue {}

// https://drafts.csswg.org/css-ruby-1/#ruby-merge
#[value(" separate | merge | auto ")]
#[initial("separate")]
#[applies_to("interlinear ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RubyMergeStyleValue {}

// https://drafts.csswg.org/css-ruby-1/#ruby-align
#[value(" start | center | space-between | space-around ")]
#[initial("space-around")]
#[applies_to("ruby bases, ruby annotations, ruby base containers, ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RubyAlignStyleValue {}

// https://drafts.csswg.org/css-ruby-1/#ruby-overhang
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RubyOverhangStyleValue {}
