mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-logical-1/
 * CSS Logical Properties and Values Level 1
 */

// https://drafts.csswg.org/css-logical-1/#block-size
#[value(" <'width'> ")]
#[initial("auto")]
#[applies_to("Same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct BlockSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#inline-size
#[value(" <'width'> ")]
#[initial("auto")]
#[applies_to("Same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct InlineSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#min-block-size
#[value(" <'min-width'> ")]
#[initial("0")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MinBlockSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#min-inline-size
#[value(" <'min-width'> ")]
#[initial("0")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MinInlineSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#max-block-size
#[value(" <'max-width'> ")]
#[initial("none")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MaxBlockSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#max-inline-size
#[value(" <'max-width'> ")]
#[initial("none")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MaxInlineSizeStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-block-start
#[value(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MarginBlockStartStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-block-end
#[value(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MarginBlockEndStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-inline-start
#[value(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MarginInlineStartStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-inline-end
#[value(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct MarginInlineEndStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-block
#[value(" <'margin-top'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct MarginBlockStyleValue;

// https://drafts.csswg.org/css-logical-1/#margin-inline
#[value(" <'margin-top'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct MarginInlineStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-block-start
#[value(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingBlockStartStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-block-end
#[value(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingBlockEndStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-inline-start
#[value(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingInlineStartStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-inline-end
#[value(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct PaddingInlineEndStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-block
#[value(" <'padding-top'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct PaddingBlockStyleValue;

// https://drafts.csswg.org/css-logical-1/#padding-inline
#[value(" <'padding-top'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct PaddingInlineStyleValue;
