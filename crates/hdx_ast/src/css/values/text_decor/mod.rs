mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-text-decor-4/
 * CSS Text Decoration Module Level 4
 */

// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-line
// #[value(" none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no (but see prose, above)")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextDecorationLineStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-style
#[value(" solid | double | dotted | dashed | wavy ")]
#[initial("solid")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextDecorationStyleStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-color
#[value(" <color> ")]
#[initial("currentcolor")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct TextDecorationColorStyleValue;

// // https://drafts.csswg.org/css-text-decor-4/#text-decoration
// #[value(" <'text-decoration-line'> || <'text-decoration-thickness'> || <'text-decoration-style'> || <'text-decoration-color'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct TextDecorationStyleValue;

// // https://drafts.csswg.org/css-text-decor-4/#text-underline-position
// #[value(" auto | [ from-font | under ] || [ left | right ] ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextUnderlinePositionStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis-style
// #[value(" none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string> ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextEmphasisStyleStyleValue<'a> {}

// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-color
#[value(" <color> ")]
#[initial("currentcolor")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct TextEmphasisColorStyleValue;

// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis
// #[value(" <'text-emphasis-style'> || <'text-emphasis-color'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct TextEmphasisStyleValue;

// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis-position
// #[value(" [ over | under ] && [ right | left ]? ")]
// #[initial("over right")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextEmphasisPositionStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-shadow
// #[value(" none | <shadow># ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as shadow list")]
// pub enum TextShadowStyleValue<'a> {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-thickness
#[value(" auto | from-font | <length-percentage> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum TextDecorationThicknessStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-underline-offset
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum TextUnderlineOffsetStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-trim
// #[value(" <length>{1,2} | auto ")]
// #[initial("0")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum TextDecorationTrimStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip
#[value(" none | auto ")]
#[initial("See individual properties")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextDecorationSkipStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-self
// #[value(" auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextDecorationSkipSelfStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-box
#[value(" none | all ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextDecorationSkipBoxStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-spaces
// #[value(" none | all | [ start || end ] ")]
// #[initial("start end")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextDecorationSkipSpacesStyleValue {}

// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink
#[value(" auto | none | all ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextDecorationSkipInkStyleValue {}

// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis-skip
// #[value(" spaces || punctuation || symbols || narrow ")]
// #[initial("spaces punctuation")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct TextEmphasisSkipStyleValue;
