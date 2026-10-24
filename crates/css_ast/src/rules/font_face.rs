use css_parse::DeclarationValue;
use csskit_proc_macro::syntax;

use super::prelude::*;
use crate::{CssMetadata, FontDisplayValue, Unknown};

/// <https://drafts.csswg.org/css-fonts/#font-face-rule>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.font-face"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = FontFace)]
pub struct FontFaceRule<'a> {
	#[atom(CssAtomSet::FontFace)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![AtKeyword],
	#[metadata(block)]
	pub block: FontFaceRuleBlock<'a>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceRuleBlock<'a>(pub DeclarationList<'a, FontFaceRuleStyleValue<'a>, CssMetadata>);

/// The descriptors allowed inside a [`FontFaceRule`] block.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontFaceRuleStyleValue<'a> {
	AscentOverride(FontFaceAscentOverrideDescriptor<'a>),
	DescentOverride(FontFaceDescentOverrideDescriptor<'a>),
	FontDisplay(FontDisplayValue),
	FontFamily(FontFaceFontFamilyDescriptor<'a>),
	FontFeatureSettings(FontFaceFontFeatureSettingsDescriptor<'a>),
	FontLanguageOverride(FontFaceFontLanguageOverrideDescriptor<'a>),
	FontNamedInstance(FontFaceFontNamedInstanceDescriptor<'a>),
	FontSize(FontFaceFontSizeDescriptor<'a>),
	FontStyle(FontFaceFontStyleDescriptor<'a>),
	FontVariationSettings(FontFaceFontVariationSettingsDescriptor<'a>),
	FontWeight(FontFaceFontWeightDescriptor<'a>),
	FontWidth(FontFaceFontWidthDescriptor<'a>),
	LineGapOverride(FontFaceLineGapOverrideDescriptor<'a>),
	SizeAdjust(FontFaceSizeAdjustDescriptor<'a>),
	Src(FontFaceSrcDescriptor<'a>),
	SubscriptPositionOverride(FontFaceSubscriptPositionOverrideDescriptor<'a>),
	SubscriptSizeOverride(FontFaceSubscriptSizeOverrideDescriptor<'a>),
	SuperscriptPositionOverride(FontFaceSuperscriptPositionOverrideDescriptor<'a>),
	SuperscriptSizeOverride(FontFaceSuperscriptSizeOverrideDescriptor<'a>),
	UnicodeRange(FontFaceUnicodeRangeDescriptor<'a>),
	Unknown(Unknown<'a>),
}

impl<'a> DeclarationValue<'a, CssMetadata> for FontFaceRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(
			p.to_atom::<CssAtomSet>(c),
			CssAtomSet::AscentOverride
				| CssAtomSet::DescentOverride
				| CssAtomSet::FontDisplay
				| CssAtomSet::FontFamily
				| CssAtomSet::FontFeatureSettings
				| CssAtomSet::FontLanguageOverride
				| CssAtomSet::FontNamedInstance
				| CssAtomSet::FontSize
				| CssAtomSet::FontStyle
				| CssAtomSet::FontVariationSettings
				| CssAtomSet::FontWeight
				| CssAtomSet::FontWidth
				| CssAtomSet::LineGapOverride
				| CssAtomSet::SizeAdjust
				| CssAtomSet::Src
				| CssAtomSet::SubscriptPositionOverride
				| CssAtomSet::SubscriptSizeOverride
				| CssAtomSet::SuperscriptPositionOverride
				| CssAtomSet::SuperscriptSizeOverride
				| CssAtomSet::UnicodeRange
		)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::AscentOverride => Self::AscentOverride(p.parse::<FontFaceAscentOverrideDescriptor<'a>>()?),
			CssAtomSet::DescentOverride => Self::DescentOverride(p.parse::<FontFaceDescentOverrideDescriptor<'a>>()?),
			CssAtomSet::FontDisplay => Self::FontDisplay(p.parse::<FontDisplayValue>()?),
			CssAtomSet::FontFamily => Self::FontFamily(p.parse::<FontFaceFontFamilyDescriptor<'a>>()?),
			CssAtomSet::FontFeatureSettings => {
				Self::FontFeatureSettings(p.parse::<FontFaceFontFeatureSettingsDescriptor<'a>>()?)
			}
			CssAtomSet::FontLanguageOverride => {
				Self::FontLanguageOverride(p.parse::<FontFaceFontLanguageOverrideDescriptor<'a>>()?)
			}
			CssAtomSet::FontNamedInstance => {
				Self::FontNamedInstance(p.parse::<FontFaceFontNamedInstanceDescriptor<'a>>()?)
			}
			CssAtomSet::FontSize => Self::FontSize(p.parse::<FontFaceFontSizeDescriptor<'a>>()?),
			CssAtomSet::FontStyle => Self::FontStyle(p.parse::<FontFaceFontStyleDescriptor<'a>>()?),
			CssAtomSet::FontVariationSettings => {
				Self::FontVariationSettings(p.parse::<FontFaceFontVariationSettingsDescriptor<'a>>()?)
			}
			CssAtomSet::FontWeight => Self::FontWeight(p.parse::<FontFaceFontWeightDescriptor<'a>>()?),
			CssAtomSet::FontWidth => Self::FontWidth(p.parse::<FontFaceFontWidthDescriptor<'a>>()?),
			CssAtomSet::LineGapOverride => Self::LineGapOverride(p.parse::<FontFaceLineGapOverrideDescriptor<'a>>()?),
			CssAtomSet::SizeAdjust => Self::SizeAdjust(p.parse::<FontFaceSizeAdjustDescriptor<'a>>()?),
			CssAtomSet::Src => Self::Src(p.parse::<FontFaceSrcDescriptor<'a>>()?),
			CssAtomSet::SubscriptPositionOverride => {
				Self::SubscriptPositionOverride(p.parse::<FontFaceSubscriptPositionOverrideDescriptor<'a>>()?)
			}
			CssAtomSet::SubscriptSizeOverride => {
				Self::SubscriptSizeOverride(p.parse::<FontFaceSubscriptSizeOverrideDescriptor<'a>>()?)
			}
			CssAtomSet::SuperscriptPositionOverride => {
				Self::SuperscriptPositionOverride(p.parse::<FontFaceSuperscriptPositionOverrideDescriptor<'a>>()?)
			}
			CssAtomSet::SuperscriptSizeOverride => {
				Self::SuperscriptSizeOverride(p.parse::<FontFaceSuperscriptSizeOverrideDescriptor<'a>>()?)
			}
			CssAtomSet::UnicodeRange => Self::UnicodeRange(p.parse::<FontFaceUnicodeRangeDescriptor<'a>>()?),
			_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
		})
	}

	fn parse_unknown_declaration_value<I>(p: &mut Parser<'a, I>, _: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Unknown>().map(Self::Unknown)
	}
}

/// ```text,ignore
/// <font-src-list>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-src>
#[syntax(" <font-src-list> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSrcDescriptor<'a>;

/// ```text,ignore
/// <unicode-range-token>#
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-unicode-range>
#[syntax(" <unicode-range># ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceUnicodeRangeDescriptor<'a>;

/// ```text,ignore
/// <font-family-name>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-family>
#[syntax(" <font-family-name> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontFamilyDescriptor<'a>;

/// ```text,ignore
/// auto | normal | italic | left | right | oblique [ <angle [-90deg,90deg]>{1,2} ]?
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-style>
#[syntax(" auto | normal | italic | left | right | oblique [ <angle [-90deg,90deg]>{1,2} ]? ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontFaceFontStyleDescriptor<'a> {}

/// ```text,ignore
/// auto | <font-weight-absolute>{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-weight>
#[syntax(" auto | <font-weight-absolute>{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontWeightDescriptor<'a>;

/// ```text,ignore
/// auto | <'font-width'>{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-width>
#[syntax(" auto | <'font-width'>{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontWidthDescriptor<'a>;

/// ```text,ignore
/// auto | [ <number> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-font-size>
#[syntax(" auto | <number>{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontSizeDescriptor<'a>;

/// ```text,ignore
/// normal | <feature-tag-value>#
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-feature-settings>
#[syntax(" normal | <feature-tag-value># ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontFeatureSettingsDescriptor<'a>;

/// ```text,ignore
/// normal | [ <opentype-tag> <number> ]#
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-variation-settings>
#[syntax(" normal | [ <opentype-tag> <number> ]# ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontVariationSettingsDescriptor<'a>;

/// ```text,ignore
/// normal | <string>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-language-override>
#[syntax(" normal | <string> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontLanguageOverrideDescriptor<'a>;

/// ```text,ignore
/// auto | <string>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-4/#descdef-font-face-font-named-instance>
#[syntax(" auto | <string> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceFontNamedInstanceDescriptor<'a>;

/// ```text,ignore
/// <percentage [0,∞]>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-size-adjust>
#[syntax(" <percentage [0,∞]> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSizeAdjustDescriptor<'a>;

/// ```text,ignore
/// [ normal | <percentage [0,∞]> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-ascent-override>
#[syntax(" [ normal | <percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceAscentOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | <percentage [0,∞]> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-descent-override>
#[syntax(" [ normal | <percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceDescentOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | <percentage [0,∞]> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-line-gap-override>
#[syntax(" [ normal | <percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceLineGapOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | from-font | <percentage> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-superscript-position-override>
#[syntax(" [ normal | from-font | <percentage> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSuperscriptPositionOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | from-font | <percentage> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-subscript-position-override>
#[syntax(" [ normal | from-font | <percentage> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSubscriptPositionOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | from-font | <percentage [0,∞]> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-superscript-size-override>
#[syntax(" [ normal | from-font | <percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSuperscriptSizeOverrideDescriptor<'a>;

/// ```text,ignore
/// [ normal | from-font | <percentage [0,∞]> ]{1,2}
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-subscript-size-override>
#[syntax(" [ normal | from-font | <percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceSubscriptSizeOverrideDescriptor<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face{}");
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFaceRule,
			"@font-face{font-family:\"Gentium\";src:url(gentium.woff2)format(woff2),local(Gentium)}"
		);
		assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face{unicode-range:U+0-7F,U+4E00-9FFF}");
		assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face{font-weight:100 900;font-width:75% 125%}");
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFaceRule,
			"@font-face{ascent-override:90%;descent-override:normal 10%;line-gap-override:0%}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFaceRule,
			"@font-face{size-adjust:105%;font-display:swap;font-named-instance:\"Book\"}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFaceRule,
			"@font-face{superscript-size-override:from-font;subscript-position-override:normal 20%}"
		);
		assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face{font-style:oblique 14deg 30deg;font-size:1 2}");
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFaceRule,
			"@font-face{font-feature-settings:\"liga\" off;font-variation-settings:\"wght\" 400}"
		);
	}

	#[test]
	fn test_unknown_descriptor() {
		assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face{color:red}");
	}
}
