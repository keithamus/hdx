mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-fonts-5/
 * CSS Fonts Module Level 5
 */

// // https://drafts.csswg.org/css-fonts-5/#font-family
// #[value(" [ <family-name> | <generic-family> ]# ")]
// #[initial("depends on user agent")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontFamily<'a> {}

// // https://drafts.csswg.org/css-fonts-5/#font-weight
// #[value(" <font-weight-absolute> | bolder | lighter ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum FontWeight {}

// https://drafts.csswg.org/css-fonts-5/#font-width
#[value(" normal | <percentage [0,∞]> | ultra-condensed | extra-condensed | condensed | semi-condensed | semi-expanded | expanded | extra-expanded | ultra-expanded ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("not resolved")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum FontWidth {}

// // https://drafts.csswg.org/css-fonts-5/#font-style
// #[value(" normal | italic | oblique <angle [-90deg,90deg]>? ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type;normal animates as oblique 0deg")]
// pub enum FontStyle {}

// // https://drafts.csswg.org/css-fonts-5/#font-size
// #[value(" <absolute-size> | <relative-size> | <length-percentage [0,∞]> | math ")]
// #[initial("medium")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("refer to parent element’s font size")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// pub enum FontSize {}

// // https://drafts.csswg.org/css-fonts-5/#font-size-adjust
// #[value(" none | [ ex-height | cap-height | ch-width | ic-width | ic-height ]? [ from-font | <number [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete if the keywords differ, otherwise by computed value type")]
// pub enum FontSizeAdjust {}

// // https://drafts.csswg.org/css-fonts-5/#font
// #[value(" [ [ <'font-style'> || <font-variant-css2> || <'font-weight'> || <font-width-css3> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'># ] | <system-family-name> ")]
// #[initial("see individual properties")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum Font {}

// https://drafts.csswg.org/css-fonts-5/#font-synthesis-weight
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontSynthesisWeight {}

// https://drafts.csswg.org/css-fonts-5/#font-synthesis-style
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontSynthesisStyle {}

// https://drafts.csswg.org/css-fonts-5/#font-synthesis-small-caps
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontSynthesisSmallCaps {}

// https://drafts.csswg.org/css-fonts-5/#font-synthesis-position
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontSynthesisPosition {}

// // https://drafts.csswg.org/css-fonts-5/#font-synthesis
// #[value(" none | [ weight || style || small-caps || position] ")]
// #[initial("weight style small-caps position")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontSynthesis {}

// https://drafts.csswg.org/css-fonts-5/#font-kerning
#[value(" auto | normal | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontKerning {}

// // https://drafts.csswg.org/css-fonts-5/#font-variant-ligatures
// #[value(" normal | none | [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontVariantLigatures {}

// https://drafts.csswg.org/css-fonts-5/#font-variant-position
#[value(" normal | sub | super ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontVariantPosition {}

// https://drafts.csswg.org/css-fonts-5/#font-variant-caps
#[value(" normal | small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontVariantCaps {}

// // https://drafts.csswg.org/css-fonts-5/#font-variant-numeric
// #[value(" normal | [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontVariantNumeric {}

// // https://drafts.csswg.org/css-fonts-5/#font-variant-alternates
// #[value(" normal | [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontVariantAlternates {}

// // https://drafts.csswg.org/css-fonts-5/#font-variant-east-asian
// #[value(" normal | [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontVariantEastAsian {}

// // https://drafts.csswg.org/css-fonts-5/#font-variant
// #[value(" normal | none | [ [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] || [ small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ] || [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] || [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] || [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] || [ sub | super ] || [ text | emoji | unicode ] ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontVariant {}

// // https://drafts.csswg.org/css-fonts-5/#font-feature-settings
// #[value(" normal | <feature-tag-value># ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum FontFeatureSettings<'a> {}

// https://drafts.csswg.org/css-fonts-5/#font-language-override
#[value(" normal | <string> ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontLanguageOverride {}

// https://drafts.csswg.org/css-fonts-5/#font-optical-sizing
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontOpticalSizing {}

// // https://drafts.csswg.org/css-fonts-5/#font-variation-settings
// #[value(" normal | [ <opentype-tag> <number>]# ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("(see prose)")]
// pub enum FontVariationSettings<'a> {}

// https://drafts.csswg.org/css-fonts-5/#font-palette
// #[value(" normal | light | dark | <palette-identifier> | <palette-mix()> ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub enum FontPalette {}

// https://drafts.csswg.org/css-fonts-5/#font-variant-emoji
#[value(" normal | text | emoji | unicode ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FontVariantEmoji {}
