mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-text-4/
 * CSS Text Module Level 4
 */

// // https://drafts.csswg.org/css-text-4/#text-transform
// #[value(" none | [capitalize | uppercase | lowercase ] || full-width || full-size-kana | math-auto ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// pub enum TextTransformStyleValue {}

// // https://drafts.csswg.org/css-text-4/#white-space
// #[value(" normal | pre | pre-wrap | pre-line | <'white-space-collapse'> || <'text-wrap-mode'> || <'white-space-trim'> ")]
// #[initial("normal")]
// #[applies_to("text")]
// #[inherited("see individual properties")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// pub enum WhiteSpaceStyleValue {}

// // https://drafts.csswg.org/css-text-4/#tab-size
// #[value(" <number [0,∞]> | <length [0,∞]> ")]
// #[initial("8")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("by computed value type")]
// pub enum TabSizeStyleValue {}

// https://drafts.csswg.org/css-text-4/#word-break
#[value(" normal | break-all | keep-all | manual | auto-phrase | break-word ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum WordBreakStyleValue {}

// https://drafts.csswg.org/css-text-4/#line-break
#[value(" auto | loose | normal | strict | anywhere ")]
#[initial("auto")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum LineBreakStyleValue {}

// https://drafts.csswg.org/css-text-4/#hyphens
#[value(" none | manual | auto ")]
#[initial("manual")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum HyphensStyleValue {}

// https://drafts.csswg.org/css-text-4/#overflow-wrap
#[value(" normal | break-word | anywhere ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum OverflowWrapStyleValue {}

// https://drafts.csswg.org/css-text-4/#word-wrap
#[value(" normal | break-word | anywhere ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum WordWrapStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-align
#[value(" start | end | left | right | center | <string> | justify | match-parent | justify-all ")]
#[initial("start")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("see individual properties")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum TextAlignStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-align-all
#[value(" start | end | left | right | center | <string> | justify | match-parent ")]
#[initial("start")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum TextAlignAllStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-align-last
#[value(" auto | start | end | left | right | center | justify | match-parent ")]
#[initial("auto")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
pub enum TextAlignLastStyleValue {}

// // https://drafts.csswg.org/css-text-4/#text-justify
// #[value(" [ auto | none | inter-word | inter-character | ruby ] || no-compress ")]
// #[initial("auto")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// pub enum TextJustifyStyleValue {}

// https://drafts.csswg.org/css-text-4/#word-spacing
#[value(" normal | <length-percentage> ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("relative to computed font-size, i.e. 1em")]
#[canonical_order("n/a")]
#[animation_type("by computed value type")]
pub enum WordSpacingStyleValue {}

// https://drafts.csswg.org/css-text-4/#letter-spacing
#[value(" normal | <length-percentage> ")]
#[initial("normal")]
#[applies_to("inline boxes and text")]
#[inherited("yes")]
#[percentages("relative to computed font-size, i.e. 1em")]
#[canonical_order("n/a")]
#[animation_type("by computed value type")]
pub enum LetterSpacingStyleValue {}

// // https://drafts.csswg.org/css-text-4/#text-indent
// #[value(" [ <length-percentage> ] && hanging? && each-line? ")]
// #[initial("0")]
// #[applies_to("block containers")]
// #[inherited("yes")]
// #[percentages("refers to block container’s own inline-axis inner size")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct TextIndentStyleValue;

// // https://drafts.csswg.org/css-text-4/#hanging-punctuation
// #[value(" none | [ first || [ force-end | allow-end ] || last ] ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum HangingPunctuationStyleValue {}

// // https://drafts.csswg.org/css-text-4/#word-space-transform
// #[value(" none | [ space | ideographic-space ] && auto-phrase? ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum WordSpaceTransformStyleValue {}

// https://drafts.csswg.org/css-text-4/#white-space-collapse
#[value(" collapse | discard | preserve | preserve-breaks | preserve-spaces | break-spaces ")]
#[initial("collapse")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum WhiteSpaceCollapseStyleValue {}

// // https://drafts.csswg.org/css-text-4/#white-space-trim
// #[value(" none | discard-before || discard-after || discard-inner ")]
// #[initial("none")]
// #[applies_to("inline boxes and block containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum WhiteSpaceTrimStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-wrap-mode
#[value(" wrap | nowrap ")]
#[initial("wrap")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextWrapModeStyleValue {}

// https://drafts.csswg.org/css-text-4/#wrap-inside
#[value(" auto | avoid ")]
#[initial("auto")]
#[applies_to("inline boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum WrapInsideStyleValue {}

// https://drafts.csswg.org/css-text-4/#wrap-before
#[value(" auto | avoid | avoid-line | avoid-flex | line | flex ")]
#[initial("auto")]
#[applies_to("inline-level boxes and flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum WrapBeforeStyleValue {}

// https://drafts.csswg.org/css-text-4/#wrap-after
#[value(" auto | avoid | avoid-line | avoid-flex | line | flex ")]
#[initial("auto")]
#[applies_to("inline-level boxes and flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum WrapAfterStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-wrap-style
#[value(" auto | balance | stable | pretty | avoid-orphans ")]
#[initial("auto")]
#[applies_to("block containers hat establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextWrapStyleStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-wrap
#[value(" <'text-wrap-mode'> || <'text-wrap-style'> ")]
#[initial("wrap")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct TextWrapStyleValue;

// https://drafts.csswg.org/css-text-4/#hyphenate-character
#[value(" auto | <string> ")]
#[initial("auto")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum HyphenateCharacterStyleValue {}

// https://drafts.csswg.org/css-text-4/#hyphenate-limit-zone
#[value(" <length-percentage> ")]
#[initial("0")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("refers to length of the line box")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct HyphenateLimitZoneStyleValue;

// // https://drafts.csswg.org/css-text-4/#hyphenate-limit-chars
// #[value(" [ auto | <integer> ]{1,3} ")]
// #[initial("auto")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum HyphenateLimitCharsStyleValue {}

// https://drafts.csswg.org/css-text-4/#hyphenate-limit-lines
#[value(" no-limit | <integer> ")]
#[initial("no-limit")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum HyphenateLimitLinesStyleValue {}

// https://drafts.csswg.org/css-text-4/#hyphenate-limit-last
#[value(" none | always | column | page | spread ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum HyphenateLimitLastStyleValue {}

// https://drafts.csswg.org/css-text-4/#text-group-align
#[value(" none | start | end | left | right | center ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum TextGroupAlignStyleValue {}

// https://drafts.csswg.org/css-text-4/#line-padding
#[value(" <length> ")]
#[initial("0")]
#[applies_to("inline boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct LinePaddingStyleValue;

// // https://drafts.csswg.org/css-text-4/#text-autospace
// #[value(" normal | <autospace> | auto ")]
// #[initial("normal")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextAutospaceStyleValue {}

// // https://drafts.csswg.org/css-text-4/#text-spacing-trim
// #[value(" <spacing-trim> | auto ")]
// #[initial("normal")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextSpacingTrimStyleValue {}

// // https://drafts.csswg.org/css-text-4/#text-spacing
// #[value(" none | auto | <spacing-trim> || <autospace> ")]
// #[initial("see individual properties")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum TextSpacingStyleValue {}
