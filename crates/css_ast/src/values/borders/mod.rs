mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-borders-4/
 * CSS Borders and Box Decorations Module Level 4
 */

// https://drafts.csswg.org/css-borders-4/#border-top-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderTopColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-right-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderRightColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-bottom-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBottomColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-left-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderLeftColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-block-start-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBlockStartColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-block-end-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBlockEndColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-inline-start-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderInlineStartColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-inline-end-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderInlineEndColorStyleValue<'a> {}

// // https://drafts.csswg.org/css-borders-4/#border-color
// #[value(" [ <color> | <image-1D> ]{1,4} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum BorderColorStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#border-block-color
#[value(" <'border-top-color'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockColorStyleValue<'a>;

// https://drafts.csswg.org/css-borders-4/#border-inline-color
#[value(" <'border-top-color'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineColorStyleValue<'a>;

// https://drafts.csswg.org/css-borders-4/#border-top-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderTopStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-right-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderRightStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-bottom-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBottomStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-left-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderLeftStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-start-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBlockStartStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-end-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBlockEndStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-start-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderInlineStartStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-end-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderInlineEndStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-style
#[value(" <'border-top-style'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-style
#[value(" <'border-top-style'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineStyleStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-top-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-right-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderRightWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-bottom-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-left-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderLeftWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-start-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBlockStartWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-end-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBlockEndWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-start-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderInlineStartWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-end-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderInlineEndWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-width
#[value(" <'border-top-width'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-width
#[value(" <'border-top-width'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineWidthStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-top
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderTopStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-right
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderRightStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-bottom
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBottomStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-left
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderLeftStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-start
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockStartStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block-end
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockEndStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-start
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineStartStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline-end
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineEndStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-block
#[value(" <'border-block-start'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-inline
#[value(" <'border-block-start'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-top-left-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopLeftRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-top-right-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopRightRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomRightRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomLeftRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-start-start-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderStartStartRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-start-end-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderStartEndRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-end-start-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderEndStartRadiusStyleValue;

// https://drafts.csswg.org/css-borders-4/#border-end-end-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderEndEndRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-top-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderTopRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-right-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderRightRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-bottom-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBottomRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-left-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderLeftRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-block-start-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockStartRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-block-end-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockEndRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-inline-start-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineStartRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-inline-end-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineEndRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-radius
// #[value(" <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ")]
// #[initial("0")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderRadiusStyleValue;

// // https://drafts.csswg.org/css-borders-4/#corner-shape
// #[value(" [ round | angle ]{1,4} ")]
// #[initial("round")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum CornerShapeStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#corners
// #[value(" <'corner-shape'> || <'border-radius'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct CornersStyleValue;

// // https://drafts.csswg.org/css-borders-4/#border-limit
// #[value(" all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]> ")]
// #[initial("all")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("relative to border-box")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum BorderLimitStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#border-clip
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-top
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipTopStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-right
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipRightStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-bottom
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipBottomStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-left
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipLeftStyleValue {}

// // https://drafts.csswg.org/css-borders-4/#box-shadow-color
// #[value(" <color># ")]
// #[initial("currentcolor")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct BoxShadowColorStyleValue<'a>;

// // https://drafts.csswg.org/css-borders-4/#box-shadow-offset
// #[value(" [ none | <length>{2} ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value, treating none as 0 0 when interpolated with non-none values.")]
// pub enum BoxShadowOffsetStyleValue<'a> {}

// https://drafts.csswg.org/css-borders-4/#box-shadow-blur
#[value(" <length [0,∞]># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BoxShadowBlurStyleValue<'a>;

// https://drafts.csswg.org/css-borders-4/#box-shadow-spread
#[value(" <length># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BoxShadowSpreadStyleValue<'a>;

// // https://drafts.csswg.org/css-borders-4/#box-shadow-position
// #[value(" [ outset | inset ]# ")]
// #[initial("outset")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BoxShadowPositionStyleValue<'a> {}

// // https://drafts.csswg.org/css-borders-4/#box-shadow
// #[value(" <spread-shadow># ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BoxShadowStyleValue<'a>;
