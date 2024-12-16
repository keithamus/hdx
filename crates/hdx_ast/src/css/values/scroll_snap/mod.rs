mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-scroll-snap-2/
 * CSS Scroll Snap Module Level 2
 */

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-type
// #[value(" none | [ x | y | block | inline | both ] [ mandatory | proximity ]? ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ScrollSnapTypeStyleValue {}

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding
// #[value(" [ auto | <length-percentage [0,∞]> ]{1,4} ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("relative to the corresponding dimension of the scroll container’s scrollport")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum ScrollPaddingStyleValue {}

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin
// #[value(" <length>{1,4} ")]
// #[initial("0")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct ScrollMarginStyleValue;

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-align
// #[value(" [ none | start | end | center ]{1,2} ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ScrollSnapAlignStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-stop
#[value(" normal | always ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ScrollSnapStopStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-top
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingTopStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-right
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingRightStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-bottom
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingBottomStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-left
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingLeftStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-start
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingInlineStartStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-start
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingBlockStartStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-end
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingInlineEndStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-end
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ScrollPaddingBlockEndStyleValue {}

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block
// #[value(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("relative to the scroll container’s scrollport")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum ScrollPaddingBlockStyleValue {}

// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline
// #[value(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("relative to the scroll container’s scrollport")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum ScrollPaddingInlineStyleValue {}

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-top
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginTopStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-right
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginRightStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-bottom
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginBottomStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-left
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginLeftStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-start
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginBlockStartStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-start
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginInlineStartStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-end
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginBlockEndStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-end
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginInlineEndStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block
#[value(" <length>{1,2} ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginBlockStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline
#[value(" <length>{1,2} ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ScrollMarginInlineStyleValue;

// https://drafts.csswg.org/css-scroll-snap-2/#scroll-start-target
#[value(" none | auto ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("none")]
pub enum ScrollStartTargetStyleValue {}
