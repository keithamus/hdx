mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-overflow-5/
 * CSS Overflow Module Level 5
 */

// https://drafts.csswg.org/css-overflow-5/#overflow-x
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverflowXStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#overflow-y
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverflowYStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#overflow-block
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverflowBlockStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#overflow-inline
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum OverflowInlineStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#overflow
#[value(" <'overflow-block'>{1,2} ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], and grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub struct OverflowStyleValue;

// // https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin
// #[value(" <visual-box> || <length [0,∞]> ")]
// #[initial("0px")]
// #[applies_to("boxes to which overflow applies")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct OverflowClipMarginStyleValue;

// https://drafts.csswg.org/css-overflow-5/#scroll-behavior
#[value(" auto | smooth ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum ScrollBehaviorStyleValue {}

// // https://drafts.csswg.org/css-overflow-5/#scrollbar-gutter
// #[value(" auto | stable && both-edges? ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ScrollbarGutterStyleValue {}

// // https://drafts.csswg.org/css-overflow-5/#text-overflow
// #[value(" [ clip | ellipsis | <string> | fade | <fade()> ]{1,2} ")]
// #[initial("clip")]
// #[applies_to("block containers")]
// #[inherited("no")]
// #[percentages("refer to the width of the line box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum TextOverflowStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-top
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginTopStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-right
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginRightStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-bottom
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBottomStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-left
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginLeftStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBlockStartStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginInlineStartStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBlockEndStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginInlineEndStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct OverflowClipMarginInlineStyleValue;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct OverflowClipMarginBlockStyleValue;

// https://drafts.csswg.org/css-overflow-5/#block-ellipsis
#[value(" none | auto | <string> ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockEllipsisStyleValue {}

// // https://drafts.csswg.org/css-overflow-5/#line-clamp
// #[value(" none | <integer [1,∞]> || <'block-ellipsis'> ")]
// #[initial("none")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum LineClampStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#-webkit-line-clamp
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub enum WebkitLineClampStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#max-lines
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("block containers which are also fragmentation containers that capture region breaks")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MaxLinesStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#continue
#[value(" auto | discard ")]
#[initial("auto")]
#[applies_to("block containers and multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ContinueStyleValue {}

// https://drafts.csswg.org/css-overflow-5/#scroll-marker-group
#[value(" none | before | after ")]
#[initial("none")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ScrollMarkerGroupStyleValue {}
