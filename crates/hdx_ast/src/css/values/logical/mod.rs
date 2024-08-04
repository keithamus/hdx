mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-logical-1/
 * CSS Logical Properties and Values Level 1
 */

// // https://drafts.csswg.org/css-logical-1/#block-size
// #[value(" <'width'> ")]
// #[initial("auto")]
// #[applies_to("Same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BlockSize;

// // https://drafts.csswg.org/css-logical-1/#inline-size
// #[value(" <'width'> ")]
// #[initial("auto")]
// #[applies_to("Same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct InlineSize;

// // https://drafts.csswg.org/css-logical-1/#min-block-size
// #[value(" <'min-width'> ")]
// #[initial("0")]
// #[applies_to("same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MinBlockSize;

// // https://drafts.csswg.org/css-logical-1/#min-inline-size
// #[value(" <'min-width'> ")]
// #[initial("0")]
// #[applies_to("same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MinInlineSize;

// // https://drafts.csswg.org/css-logical-1/#max-block-size
// #[value(" <'max-width'> ")]
// #[initial("none")]
// #[applies_to("same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MaxBlockSize;

// // https://drafts.csswg.org/css-logical-1/#max-inline-size
// #[value(" <'max-width'> ")]
// #[initial("none")]
// #[applies_to("same as height and width")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MaxInlineSize;

// // https://drafts.csswg.org/css-logical-1/#margin-block-start
// #[value(" <'margin-top'> ")]
// #[initial("0")]
// #[applies_to("Same as margin-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MarginBlockStart;

// // https://drafts.csswg.org/css-logical-1/#margin-block-end
// #[value(" <'margin-top'> ")]
// #[initial("0")]
// #[applies_to("Same as margin-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MarginBlockEnd;

// // https://drafts.csswg.org/css-logical-1/#margin-inline-start
// #[value(" <'margin-top'> ")]
// #[initial("0")]
// #[applies_to("Same as margin-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MarginInlineStart;

// // https://drafts.csswg.org/css-logical-1/#margin-inline-end
// #[value(" <'margin-top'> ")]
// #[initial("0")]
// #[applies_to("Same as margin-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct MarginInlineEnd;

// // https://drafts.csswg.org/css-logical-1/#margin-block
// #[value(" <'margin-top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct MarginBlock;

// // https://drafts.csswg.org/css-logical-1/#margin-inline
// #[value(" <'margin-top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct MarginInline;

// // https://drafts.csswg.org/css-logical-1/#inset-block-start
// #[value(" <'top'> ")]
// #[initial("auto")]
// #[applies_to("positioned elements")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct InsetBlockStart;

// // https://drafts.csswg.org/css-logical-1/#inset-block-end
// #[value(" <'top'> ")]
// #[initial("auto")]
// #[applies_to("positioned elements")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct InsetBlockEnd;

// // https://drafts.csswg.org/css-logical-1/#inset-inline-start
// #[value(" <'top'> ")]
// #[initial("auto")]
// #[applies_to("positioned elements")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct InsetInlineStart;

// // https://drafts.csswg.org/css-logical-1/#inset-inline-end
// #[value(" <'top'> ")]
// #[initial("auto")]
// #[applies_to("positioned elements")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct InsetInlineEnd;

// // https://drafts.csswg.org/css-logical-1/#inset-block
// #[value(" <'top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct InsetBlock;

// // https://drafts.csswg.org/css-logical-1/#inset-inline
// #[value(" <'top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct InsetInline;

// // https://drafts.csswg.org/css-logical-1/#inset
// #[value(" <'top'>{1,4} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct Inset;

// // https://drafts.csswg.org/css-logical-1/#padding-block-start
// #[value(" <'padding-top'> ")]
// #[initial("0")]
// #[applies_to("Same as padding-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct PaddingBlockStart;

// // https://drafts.csswg.org/css-logical-1/#padding-block-end
// #[value(" <'padding-top'> ")]
// #[initial("0")]
// #[applies_to("Same as padding-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct PaddingBlockEnd;

// // https://drafts.csswg.org/css-logical-1/#padding-inline-start
// #[value(" <'padding-top'> ")]
// #[initial("0")]
// #[applies_to("Same as padding-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct PaddingInlineStart;

// // https://drafts.csswg.org/css-logical-1/#padding-inline-end
// #[value(" <'padding-top'> ")]
// #[initial("0")]
// #[applies_to("Same as padding-top")]
// #[inherited("no")]
// #[percentages("as for the corresponding physical property")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct PaddingInlineEnd;

// // https://drafts.csswg.org/css-logical-1/#padding-block
// #[value(" <'padding-top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct PaddingBlock;

// // https://drafts.csswg.org/css-logical-1/#padding-inline
// #[value(" <'padding-top'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct PaddingInline;

// // https://drafts.csswg.org/css-logical-1/#border-block-start-width
// #[value(" <'border-top-width'> ")]
// #[initial("medium")]
// #[applies_to("Same as border-top-width")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderBlockStartWidth;

// // https://drafts.csswg.org/css-logical-1/#border-block-end-width
// #[value(" <'border-top-width'> ")]
// #[initial("medium")]
// #[applies_to("Same as border-top-width")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderBlockEndWidth;

// // https://drafts.csswg.org/css-logical-1/#border-inline-start-width
// #[value(" <'border-top-width'> ")]
// #[initial("medium")]
// #[applies_to("Same as border-top-width")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderInlineStartWidth;

// // https://drafts.csswg.org/css-logical-1/#border-inline-end-width
// #[value(" <'border-top-width'> ")]
// #[initial("medium")]
// #[applies_to("Same as border-top-width")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderInlineEndWidth;

// // https://drafts.csswg.org/css-logical-1/#border-block-width
// #[value(" <'border-top-width'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockWidth;

// // https://drafts.csswg.org/css-logical-1/#border-inline-width
// #[value(" <'border-top-width'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineWidth;

// // https://drafts.csswg.org/css-logical-1/#border-block-start-style
// #[value(" <'border-top-style'> ")]
// #[initial("none")]
// #[applies_to("Same as border-top-style")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BorderBlockStartStyle;

// // https://drafts.csswg.org/css-logical-1/#border-block-end-style
// #[value(" <'border-top-style'> ")]
// #[initial("none")]
// #[applies_to("Same as border-top-style")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BorderBlockEndStyle;

// // https://drafts.csswg.org/css-logical-1/#border-inline-start-style
// #[value(" <'border-top-style'> ")]
// #[initial("none")]
// #[applies_to("Same as border-top-style")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BorderInlineStartStyle;

// // https://drafts.csswg.org/css-logical-1/#border-inline-end-style
// #[value(" <'border-top-style'> ")]
// #[initial("none")]
// #[applies_to("Same as border-top-style")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct BorderInlineEndStyle;

// // https://drafts.csswg.org/css-logical-1/#border-block-style
// #[value(" <'border-top-style'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockStyle;

// // https://drafts.csswg.org/css-logical-1/#border-inline-style
// #[value(" <'border-top-style'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineStyle;

// // https://drafts.csswg.org/css-logical-1/#border-block-start-color
// #[value(" <'border-top-color'> ")]
// #[initial("currentcolor")]
// #[applies_to("Same as border-top-color")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderBlockStartColor;

// // https://drafts.csswg.org/css-logical-1/#border-block-end-color
// #[value(" <'border-top-color'> ")]
// #[initial("currentcolor")]
// #[applies_to("Same as border-top-color")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderBlockEndColor;

// // https://drafts.csswg.org/css-logical-1/#border-inline-start-color
// #[value(" <'border-top-color'> ")]
// #[initial("currentcolor")]
// #[applies_to("Same as border-top-color")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderInlineStartColor;

// // https://drafts.csswg.org/css-logical-1/#border-inline-end-color
// #[value(" <'border-top-color'> ")]
// #[initial("currentcolor")]
// #[applies_to("Same as border-top-color")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderInlineEndColor;

// // https://drafts.csswg.org/css-logical-1/#border-block-color
// #[value(" <'border-top-color'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockColor;

// // https://drafts.csswg.org/css-logical-1/#border-inline-color
// #[value(" <'border-top-color'>{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineColor;

// // https://drafts.csswg.org/css-logical-1/#border-block-start
// #[value(" <'border-top-width'> || <'border-top-style'> || <color> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockStart;

// // https://drafts.csswg.org/css-logical-1/#border-block-end
// #[value(" <'border-top-width'> || <'border-top-style'> || <color> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlockEnd;

// // https://drafts.csswg.org/css-logical-1/#border-inline-start
// #[value(" <'border-top-width'> || <'border-top-style'> || <color> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineStart;

// // https://drafts.csswg.org/css-logical-1/#border-inline-end
// #[value(" <'border-top-width'> || <'border-top-style'> || <color> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInlineEnd;

// // https://drafts.csswg.org/css-logical-1/#border-block
// #[value(" <'border-block-start'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderBlock;

// // https://drafts.csswg.org/css-logical-1/#border-inline
// #[value(" <'border-block-start'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct BorderInline;

// // https://drafts.csswg.org/css-logical-1/#border-start-start-radius
// #[value(" <'border-top-left-radius'> ")]
// #[initial("Same as border-top-left-radius")]
// #[applies_to("Same as border-top-left-radius")]
// #[inherited("no")]
// #[percentages("same as border-top-left-radius")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderStartStartRadius;

// // https://drafts.csswg.org/css-logical-1/#border-start-end-radius
// #[value(" <'border-top-left-radius'> ")]
// #[initial("Same as border-top-left-radius")]
// #[applies_to("Same as border-top-left-radius")]
// #[inherited("no")]
// #[percentages("same as border-top-left-radius")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderStartEndRadius;

// // https://drafts.csswg.org/css-logical-1/#border-end-start-radius
// #[value(" <'border-top-left-radius'> ")]
// #[initial("Same as border-top-left-radius")]
// #[applies_to("Same as border-top-left-radius")]
// #[inherited("no")]
// #[percentages("same as border-top-left-radius")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderEndStartRadius;

// // https://drafts.csswg.org/css-logical-1/#border-end-end-radius
// #[value(" <'border-top-left-radius'> ")]
// #[initial("Same as border-top-left-radius")]
// #[applies_to("Same as border-top-left-radius")]
// #[inherited("no")]
// #[percentages("same as border-top-left-radius")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub struct BorderEndEndRadius;
