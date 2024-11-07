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
pub enum BorderTopColor {}

// https://drafts.csswg.org/css-borders-4/#border-right-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderRightColor {}

// https://drafts.csswg.org/css-borders-4/#border-bottom-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBottomColor {}

// https://drafts.csswg.org/css-borders-4/#border-left-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderLeftColor {}

// https://drafts.csswg.org/css-borders-4/#border-block-start-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBlockStartColor {}

// https://drafts.csswg.org/css-borders-4/#border-block-end-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderBlockEndColor {}

// https://drafts.csswg.org/css-borders-4/#border-inline-start-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderInlineStartColor {}

// https://drafts.csswg.org/css-borders-4/#border-inline-end-color
#[value(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
pub enum BorderInlineEndColor {}

// // https://drafts.csswg.org/css-borders-4/#border-color
// #[value(" [ <color> | <image-1D> ]{1,4} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum BorderColor {}

// https://drafts.csswg.org/css-borders-4/#border-block-color
#[value(" <'border-top-color'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockColor;

// https://drafts.csswg.org/css-borders-4/#border-inline-color
#[value(" <'border-top-color'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineColor;

// https://drafts.csswg.org/css-borders-4/#border-top-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderTopStyle;

// https://drafts.csswg.org/css-borders-4/#border-right-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderRightStyle;

// https://drafts.csswg.org/css-borders-4/#border-bottom-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBottomStyle;

// https://drafts.csswg.org/css-borders-4/#border-left-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderLeftStyle;

// https://drafts.csswg.org/css-borders-4/#border-block-start-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBlockStartStyle;

// https://drafts.csswg.org/css-borders-4/#border-block-end-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderBlockEndStyle;

// https://drafts.csswg.org/css-borders-4/#border-inline-start-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderInlineStartStyle;

// https://drafts.csswg.org/css-borders-4/#border-inline-end-style
#[value(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct BorderInlineEndStyle;

// https://drafts.csswg.org/css-borders-4/#border-block-style
#[value(" <'border-top-style'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockStyle;

// https://drafts.csswg.org/css-borders-4/#border-inline-style
#[value(" <'border-top-style'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineStyle;

// https://drafts.csswg.org/css-borders-4/#border-top-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopWidth;

// https://drafts.csswg.org/css-borders-4/#border-right-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderRightWidth;

// https://drafts.csswg.org/css-borders-4/#border-bottom-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomWidth;

// https://drafts.csswg.org/css-borders-4/#border-left-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderLeftWidth;

// https://drafts.csswg.org/css-borders-4/#border-block-start-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBlockStartWidth;

// https://drafts.csswg.org/css-borders-4/#border-block-end-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBlockEndWidth;

// https://drafts.csswg.org/css-borders-4/#border-inline-start-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderInlineStartWidth;

// https://drafts.csswg.org/css-borders-4/#border-inline-end-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderInlineEndWidth;

// https://drafts.csswg.org/css-borders-4/#border-block-width
#[value(" <'border-top-width'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockWidth;

// https://drafts.csswg.org/css-borders-4/#border-inline-width
#[value(" <'border-top-width'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineWidth;

// https://drafts.csswg.org/css-borders-4/#border-top
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderTop;

// https://drafts.csswg.org/css-borders-4/#border-right
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderRight;

// https://drafts.csswg.org/css-borders-4/#border-bottom
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBottom;

// https://drafts.csswg.org/css-borders-4/#border-left
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderLeft;

// https://drafts.csswg.org/css-borders-4/#border-block-start
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockStart;

// https://drafts.csswg.org/css-borders-4/#border-block-end
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlockEnd;

// https://drafts.csswg.org/css-borders-4/#border-inline-start
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineStart;

// https://drafts.csswg.org/css-borders-4/#border-inline-end
#[value(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInlineEnd;

// https://drafts.csswg.org/css-borders-4/#border-block
#[value(" <'border-block-start'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderBlock;

// https://drafts.csswg.org/css-borders-4/#border-inline
#[value(" <'border-block-start'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct BorderInline;

// https://drafts.csswg.org/css-borders-4/#border-top-left-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopLeftRadius;

// https://drafts.csswg.org/css-borders-4/#border-top-right-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderTopRightRadius;

// https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomRightRadius;

// https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderBottomLeftRadius;

// https://drafts.csswg.org/css-borders-4/#border-start-start-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderStartStartRadius;

// https://drafts.csswg.org/css-borders-4/#border-start-end-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderStartEndRadius;

// https://drafts.csswg.org/css-borders-4/#border-end-start-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderEndStartRadius;

// https://drafts.csswg.org/css-borders-4/#border-end-end-radius
#[value(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BorderEndEndRadius;

// // https://drafts.csswg.org/css-borders-4/#border-top-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderTopRadius;

// // https://drafts.csswg.org/css-borders-4/#border-right-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderRightRadius;

// // https://drafts.csswg.org/css-borders-4/#border-bottom-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBottomRadius;

// // https://drafts.csswg.org/css-borders-4/#border-left-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderLeftRadius;

// // https://drafts.csswg.org/css-borders-4/#border-block-start-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockStartRadius;

// // https://drafts.csswg.org/css-borders-4/#border-block-end-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockEndRadius;

// // https://drafts.csswg.org/css-borders-4/#border-inline-start-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineStartRadius;

// // https://drafts.csswg.org/css-borders-4/#border-inline-end-radius
// #[value(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineEndRadius;

// // https://drafts.csswg.org/css-borders-4/#border-radius
// #[value(" <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ")]
// #[initial("0")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderRadius;

// // https://drafts.csswg.org/css-borders-4/#corner-shape
// #[value(" [ round | angle ]{1,4} ")]
// #[initial("round")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum CornerShape {}

// // https://drafts.csswg.org/css-borders-4/#corners
// #[value(" <'corner-shape'> || <'border-radius'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct Corners;

// // https://drafts.csswg.org/css-borders-4/#border-limit
// #[value(" all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]> ")]
// #[initial("all")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("relative to border-box")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum BorderLimit {}

// // https://drafts.csswg.org/css-borders-4/#border-clip
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClip {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-top
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipTop {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-right
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipRight {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-bottom
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipBottom {}

// // https://drafts.csswg.org/css-borders-4/#border-clip-left
// #[value(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to length of border-edge side")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BorderClipLeft {}

// // https://drafts.csswg.org/css-borders-4/#box-shadow-color
// #[value(" <color># ")]
// #[initial("currentcolor")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct BoxShadowColor;

// // https://drafts.csswg.org/css-borders-4/#box-shadow-offset
// #[value(" [ none | <length>{2} ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value,
// treating none as 0 0 when interpolated with non-none values.")]
// pub enum BoxShadowOffset {}

// https://drafts.csswg.org/css-borders-4/#box-shadow-blur
#[value(" <length [0,∞]># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BoxShadowBlur;

// https://drafts.csswg.org/css-borders-4/#box-shadow-spread
#[value(" <length># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct BoxShadowSpread;

// // https://drafts.csswg.org/css-borders-4/#box-shadow-position
// #[value(" [ outset | inset ]# ")]
// #[initial("outset")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum BoxShadowPosition {}

// // https://drafts.csswg.org/css-borders-4/#box-shadow
// #[value(" <spread-shadow># ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BoxShadow;
