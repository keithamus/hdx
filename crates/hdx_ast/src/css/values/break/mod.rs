mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-break-4/
 * CSS Fragmentation Module Level 4  Breaking the Web, one fragment at a time
 */

// https://drafts.csswg.org/css-break-4/#break-before
#[value(" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region ")]
#[initial("auto")]
#[applies_to("block-level boxes, grid items, flex items, table row groups, table rows (but see prose)")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BreakBefore {}

// https://drafts.csswg.org/css-break-4/#break-after
#[value(" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region ")]
#[initial("auto")]
#[applies_to("block-level boxes, grid items, flex items, table row groups, table rows (but see prose)")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BreakAfter {}

// https://drafts.csswg.org/css-break-4/#break-inside
#[value(" auto | avoid | avoid-page | avoid-column | avoid-region ")]
#[initial("auto")]
#[applies_to("all elements except inline-level boxes, internal ruby boxes, table column boxes, table column group boxes, absolutely-positioned boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BreakInside {}

// https://drafts.csswg.org/css-break-4/#orphans
#[value(" <integer [1,∞]> ")]
#[initial("2")]
#[applies_to("block containers that establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct Orphans;

// https://drafts.csswg.org/css-break-4/#widows
#[value(" <integer [1,∞]> ")]
#[initial("2")]
#[applies_to("block containers that establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct Widows;

// https://drafts.csswg.org/css-break-4/#box-decoration-break
#[value(" slice | clone ")]
#[initial("slice")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BoxDecorationBreak {}

// https://drafts.csswg.org/css-break-4/#margin-break
#[value(" auto | keep | discard ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum MarginBreak {}
