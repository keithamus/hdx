mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-backgrounds-4/
 * CSS Backgrounds Module Level 4
 */

// https://drafts.csswg.org/css-backgrounds-4/#background-color
#[value(" <color> ")]
#[initial("transparent")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BackgroundColorStyleValue;

// // https://drafts.csswg.org/css-backgrounds-4/#background-image
// #[value(" <bg-image># ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BackgroundImageStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
#[value(" <repeat-style># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundRepeatStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-attachment
#[value(" <attachment># ")]
#[initial("scroll")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundAttachmentStyleValue<'a>;

// // https://drafts.csswg.org/css-backgrounds-4/#background-position
// #[value(" <bg-position># ")]
// #[initial("0% 0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to size of background positioning area minus size of background image; see text")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub struct BackgroundPositionStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-clip
#[value(" <bg-clip># ")]
#[initial("border-box")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("repeatable list")]
pub struct BackgroundClipStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-origin
#[value(" <visual-box># ")]
#[initial("padding-box")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("repeatable list")]
pub struct BackgroundOriginStyleValue<'a>;

// // https://drafts.csswg.org/css-backgrounds-4/#background-size
// #[value(" <bg-size># ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("see text")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub struct BackgroundSizeStyleValue<'a>;

// // https://drafts.csswg.org/css-backgrounds-4/#background
// #[value(" <bg-layer>#? , <final-bg-layer> ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BackgroundStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#border-image-source
#[value(" none | <image> ")]
#[initial("none")]
#[applies_to("All elements, except internal table elements when border-collapse is collapse")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BorderImageSourceStyleValue<'a> {}

// // https://drafts.csswg.org/css-backgrounds-4/#border-image-slice
// #[value(" [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill? ")]
// #[initial("100%")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("refer to size of the border image")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderImageSliceStyleValue {}

// // https://drafts.csswg.org/css-backgrounds-4/#border-image-width
// #[value(" [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4} ")]
// #[initial("1")]
// #[applies_to("All elements,except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("relative to width/height of the border image area")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderImageWidthStyleValue {}

// // https://drafts.csswg.org/css-backgrounds-4/#border-image-outset
// #[value(" [ <length [0,∞]> | <number [0,∞]> ]{1,4} ")]
// #[initial("0")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderImageOutsetStyleValue {}

// // https://drafts.csswg.org/css-backgrounds-4/#border-image-repeat
// #[value(" [ stretch | repeat | round | space ]{1,2} ")]
// #[initial("stretch")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum BorderImageRepeatStyleValue {}

// // https://drafts.csswg.org/css-backgrounds-4/#border-image
// #[value(" <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'> ")]
// #[initial("See individual properties")]
// #[applies_to("See individual properties")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum BorderImageStyleValue {}

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-x
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundRepeatXStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-y
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundRepeatYStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-block
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundRepeatBlockStyleValue<'a>;

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-inline
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BackgroundRepeatInlineStyleValue<'a>;

// // https://drafts.csswg.org/css-backgrounds-4/#background-position-x
// #[value(" [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to width of background positioning area minus width of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub enum BackgroundPositionXStyleValue<'a> {}

// // https://drafts.csswg.org/css-backgrounds-4/#background-position-y
// #[value(" [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to height of background positioning area minus height of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub enum BackgroundPositionYStyleValue<'a> {}

// // https://drafts.csswg.org/css-backgrounds-4/#background-position-inline
// #[value(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to inline-size of background positioning area minus inline-size of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub enum BackgroundPositionInlineStyleValue<'a> {}

// // https://drafts.csswg.org/css-backgrounds-4/#background-position-block
// #[value(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to size of background positioning area minus size of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// pub enum BackgroundPositionBlockStyleValue<'a> {}
