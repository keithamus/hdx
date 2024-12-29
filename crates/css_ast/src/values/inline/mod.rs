mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-inline-3/
 * CSS Inline Layout Module Level 3
 */

// https://drafts.csswg.org/css-inline-3/#dominant-baseline
#[value(" auto | text-bottom | alphabetic | ideographic | middle | central | mathematical | hanging | text-top ")]
#[initial("auto")]
#[applies_to(
	"block containers, inline boxes, table rows, grid containers, flex containers, and SVG text content elements"
)]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum DominantBaselineStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#vertical-align
// #[value(" [ first | last] || <'alignment-baseline'> || <'baseline-shift'> ")]
// #[initial("baseline")]
// #[applies_to("see individual properties")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum VerticalAlignStyleValue {}

// https://drafts.csswg.org/css-inline-3/#baseline-source
#[value(" auto | first | last ")]
#[initial("auto")]
#[applies_to("inline-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BaselineSourceStyleValue {}

// https://drafts.csswg.org/css-inline-3/#alignment-baseline
#[value(" baseline | text-bottom | alphabetic | ideographic | middle | central | mathematical | text-top ")]
#[initial("baseline")]
#[applies_to("inline-level boxes, flex items, grid items, table cells, and SVG text content elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AlignmentBaselineStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#baseline-shift
// #[value(" <length-percentage> | sub | super | top | center | bottom ")]
// #[initial("0")]
// #[applies_to("inline-level boxes and SVG text content elements")]
// #[inherited("no")]
// #[percentages("refer to the used value of line-height")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum BaselineShiftStyleValue {}

// https://drafts.csswg.org/css-inline-3/#line-height
#[value(" normal | <number [0,∞]> | <length-percentage [0,∞]> ")]
#[initial("normal")]
#[applies_to("non-replaced inline boxes and SVG text content elements")]
#[inherited("yes")]
#[percentages("computed relative to 1em")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum LineHeightStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#line-fit-edge
// #[value(" leading | <text-edge> ")]
// #[initial("leading")]
// #[applies_to("inline boxes")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum LineFitEdgeStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#text-box
// #[value(" normal | <'text-box-trim'> || <'text-box-edge'> ")]
// #[initial("normal")]
// #[applies_to("block containers and inline boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextBoxStyleValue {}

// https://drafts.csswg.org/css-inline-3/#text-box-trim
#[value(" none | trim-start | trim-end | trim-both ")]
#[initial("none")]
#[applies_to("block containers and inline boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextBoxTrimStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#text-box-edge
// #[value(" auto | <text-edge> ")]
// #[initial("auto")]
// #[applies_to("block containers and inline boxes")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextBoxEdgeStyleValue {}

// https://drafts.csswg.org/css-inline-3/#inline-sizing
#[value(" normal | stretch ")]
#[initial("normal")]
#[applies_to("inline boxes, but not ruby container boxes nor internal ruby boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum InlineSizingStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#initial-letter
// #[value(" normal | <number [1,∞]> <integer [1,∞]> | <number [1,∞]> && [ drop | raise ]? ")]
// #[initial("normal")]
// #[applies_to("certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum InitialLetterStyleValue {}

// // https://drafts.csswg.org/css-inline-3/#initial-letter-align
// #[value(" [ border-box? [ alphabetic | ideographic | hanging | leading ]? ]! ")]
// #[initial("alphabetic")]
// #[applies_to("certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum InitialLetterAlignStyleValue {}

// https://drafts.csswg.org/css-inline-3/#initial-letter-wrap
#[value(" none | first | all | grid | <length-percentage> ")]
#[initial("none")]
#[applies_to("certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)")]
#[inherited("yes")]
#[percentages("relative to logical width of (last fragment of) initial letter")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InitialLetterWrapStyleValue {}
