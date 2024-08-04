mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-overflow-5/
 * CSS Overflow Module Level 5
 */

// // https://drafts.csswg.org/css-overflow-5/#overflow-x
// #[value(" visible | hidden | clip | scroll | auto ")]
// #[initial("visible")]
// #[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum OverflowX {}

// // https://drafts.csswg.org/css-overflow-5/#overflow-y
// #[value(" visible | hidden | clip | scroll | auto ")]
// #[initial("visible")]
// #[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum OverflowY {}

// // https://drafts.csswg.org/css-overflow-5/#overflow-block
// #[value(" visible | hidden | clip | scroll | auto ")]
// #[initial("visible")]
// #[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum OverflowBlock {}

// // https://drafts.csswg.org/css-overflow-5/#overflow-inline
// #[value(" visible | hidden | clip | scroll | auto ")]
// #[initial("visible")]
// #[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum OverflowInline {}

// // https://drafts.csswg.org/css-overflow-5/#overflow
// #[value(" <'overflow-block'>{1,2} ")]
// #[initial("visible")]
// #[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], and grid containers [CSS3-GRID-LAYOUT]")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct Overflow;

// // https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin
// #[value(" <visual-box> || <length [0,∞]> ")]
// #[initial("0px")]
// #[applies_to("boxes to which overflow applies")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct OverflowClipMargin;

// https://drafts.csswg.org/css-overflow-5/#scroll-behavior
#[value(" auto | smooth ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum ScrollBehavior {}

// // https://drafts.csswg.org/css-overflow-5/#scrollbar-gutter
// #[value(" auto | stable && both-edges? ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ScrollbarGutter {}

// // https://drafts.csswg.org/css-overflow-5/#text-overflow
// #[value(" [ clip | ellipsis | <string> | fade | <fade()> ]{1,2} ")]
// #[initial("clip")]
// #[applies_to("block containers")]
// #[inherited("no")]
// #[percentages("refer to the width of the line box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum TextOverflow {}

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-top
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginTop;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-right
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginRight;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-bottom
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBottom;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-left
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginLeft;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBlockStart;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginInlineStart;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginBlockEnd;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
pub struct OverflowClipMarginInlineEnd;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct OverflowClipMarginInline;

// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct OverflowClipMarginBlock;

// https://drafts.csswg.org/css-overflow-5/#block-ellipsis
#[value(" none | auto | <string> ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum BlockEllipsis<'a> {}

// // https://drafts.csswg.org/css-overflow-5/#line-clamp
// #[value(" none | <integer [1,∞]> || <'block-ellipsis'> ")]
// #[initial("none")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum LineClamp {}

// https://drafts.csswg.org/css-overflow-5/#-webkit-line-clamp
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub enum WebkitLineClamp {}

// https://drafts.csswg.org/css-overflow-5/#max-lines
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("block containers which are also fragmentation containers that capture region breaks")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum MaxLines {}

// https://drafts.csswg.org/css-overflow-5/#continue
#[value(" auto | discard ")]
#[initial("auto")]
#[applies_to("block containers and multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum Continue {}

// https://drafts.csswg.org/css-overflow-5/#scroll-marker-group
#[value(" none | before | after ")]
#[initial("none")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ScrollMarkerGroup {}
