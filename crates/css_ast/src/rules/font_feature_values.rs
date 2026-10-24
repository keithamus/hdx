use super::prelude::*;
use crate::{FontFamilyName, UnknownAtRule};
use css_parse::RuleVariants;
use csskit_proc_macro::syntax;

/// Represents the `@font-feature-values` at-rule, e.g. `@font-feature-values Taisho Gothic { @annotation { boxed: 1 } }`.
///
/// ```md
/// <font-feature-values-rule>
///  │├─ "@font-feature-values" ─╭─ <font-family-name> ─ "," ─╮─ <block> ─┤│
///                              ╰────────────────────────────╯
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#at-ruledef-font-feature-values>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.font-feature-values")
)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = FontFeatureValues)]
pub struct FontFeatureValuesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::FontFeatureValues)]
	pub name: T![AtKeyword],
	pub prelude: FontFeatureValuesPrelude<'a>,
	#[metadata(block)]
	pub block: FontFeatureValuesRuleBlock<'a>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureValuesPrelude<'a>(pub CommaSeparated<'a, FontFamilyName<'a>>);

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureValuesRuleBlock<'a>(
	pub Block<'a, FontFeatureValuesRuleStyleValue<'a>, FontFeatureValueTypeRule<'a>, CssMetadata>,
);

/// The descriptors allowed directly inside an [`FontFeatureValuesRule`] block.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontFeatureValuesRuleStyleValue<'a> {
	FontDisplay(FontDisplayValue),
	Unknown(ComponentValues<'a>),
}

impl<'a> DeclarationValue<'a, CssMetadata> for FontFeatureValuesRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::FontDisplay)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::FontDisplay => Self::FontDisplay(p.parse::<FontDisplayValue>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

/// <https://drafts.csswg.org/css-fonts/#descdef-font-feature-values-font-display>
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontDisplayValue {
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Swap)]
	Swap(T![Ident]),
	#[atom(CssAtomSet::Fallback)]
	Fallback(T![Ident]),
	#[atom(CssAtomSet::Optional)]
	Optional(T![Ident]),
}

/// The _feature value blocks_ - subsidiary at-rules of [`FontFeatureValuesRule`].
///
/// ```md
/// <font-feature-value-type>
///  │├─╭─ "@stylistic" ────────── <block> ─╮─┤│
///     ├─ "@historical-forms" ─── <block> ─┤
///     ├─ "@styleset" ─────────── <block> ─┤
///     ├─ "@character-variant" ── <block> ─┤
///     ├─ "@swash" ────────────── <block> ─┤
///     ├─ "@ornaments" ────────── <block> ─┤
///     ╰─ "@annotation" ───────── <block> ─╯
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#typedef-font-feature-values-font-feature-value-type>
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum FontFeatureValueTypeRule<'a> {
	Stylistic(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueBlock<'a>),
	HistoricalForms(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueBlock<'a>),
	Styleset(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueListBlock<'a>),
	CharacterVariant(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValuePairBlock<'a>),
	Swash(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueBlock<'a>),
	Ornaments(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueBlock<'a>),
	Annotation(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], FontFeatureValueBlock<'a>),
	UnknownAt(UnknownAtRule<'a>),
}

impl<'a> NodeWithMetadata<CssMetadata> for FontFeatureValueTypeRule<'a> {
	fn metadata(&self) -> CssMetadata {
		match self {
			Self::Stylistic(_, block)
			| Self::HistoricalForms(_, block)
			| Self::Swash(_, block)
			| Self::Ornaments(_, block)
			| Self::Annotation(_, block) => block.metadata(),
			Self::Styleset(_, block) => block.metadata(),
			Self::CharacterVariant(_, block) => block.metadata(),
			Self::UnknownAt(rule) => rule.metadata(),
		}
	}
}

impl<'a> RuleVariants<'a> for FontFeatureValueTypeRule<'a> {
	type DeclarationValue = FontFeatureValuesRuleStyleValue<'a>;
	type Metadata = CssMetadata;

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let at_keyword = p.parse::<T![AtKeyword]>()?;
		Ok(match p.to_atom::<CssAtomSet>(name) {
			CssAtomSet::Stylistic => Self::Stylistic(at_keyword, p.parse::<FontFeatureValueBlock<'a>>()?),
			CssAtomSet::HistoricalForms => Self::HistoricalForms(at_keyword, p.parse::<FontFeatureValueBlock<'a>>()?),
			CssAtomSet::Styleset => Self::Styleset(at_keyword, p.parse::<FontFeatureValueListBlock<'a>>()?),
			CssAtomSet::CharacterVariant => {
				Self::CharacterVariant(at_keyword, p.parse::<FontFeatureValuePairBlock<'a>>()?)
			}
			CssAtomSet::Swash => Self::Swash(at_keyword, p.parse::<FontFeatureValueBlock<'a>>()?),
			CssAtomSet::Ornaments => Self::Ornaments(at_keyword, p.parse::<FontFeatureValueBlock<'a>>()?),
			CssAtomSet::Annotation => Self::Annotation(at_keyword, p.parse::<FontFeatureValueBlock<'a>>()?),
			_ => Err(Diagnostic::new(name, Diagnostic::unexpected_at_rule))?,
		})
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<UnknownAtRule>().map(Self::UnknownAt)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::UnknownAt(_))
	}
}

impl<'a> Parse<'a> for FontFeatureValueTypeRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

/// The block of `@stylistic`, `@historical-forms`, `@swash`, `@ornaments` and `@annotation`, each declaration of which
/// takes a single `<font-feature-index>`.
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureValueBlock<'a>(pub DeclarationList<'a, FontFeatureValue<'a>, CssMetadata>);

/// The block of `@character-variant`, each declaration of which takes one or two `<font-feature-index>`es.
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureValuePairBlock<'a>(pub DeclarationList<'a, FontFeatureValuePair<'a>, CssMetadata>);

/// The block of `@styleset`, each declaration of which takes any number of `<font-feature-index>`es.
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureValueListBlock<'a>(pub DeclarationList<'a, FontFeatureValueList<'a>, CssMetadata>);

/// ```text,ignore
/// <font-feature-index> = <integer [0,∞]>
/// ```
#[syntax(" <integer [0,∞]> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureIndex<'a>;

/// ```text,ignore
/// <font-feature-index>{1,2}
/// ```
#[syntax(" <integer [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureIndexPair<'a>;

/// ```text,ignore
/// <font-feature-index>+
/// ```
#[syntax(" <integer [0,∞]>+ ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFeatureIndexList<'a>;

/// A _font feature value declaration_ taking a single `<font-feature-index>`.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum FontFeatureValue<'a> {
	Index(FontFeatureIndex<'a>),
	Unknown(ComponentValues<'a>),
}

/// A _font feature value declaration_ taking one or two `<font-feature-index>`es.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum FontFeatureValuePair<'a> {
	Index(FontFeatureIndexPair<'a>),
	Unknown(ComponentValues<'a>),
}

/// A _font feature value declaration_ taking any number of `<font-feature-index>`es.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum FontFeatureValueList<'a> {
	Index(FontFeatureIndexList<'a>),
	Unknown(ComponentValues<'a>),
}

macro_rules! font_feature_value {
	($value: ident, $index: ident) => {
		impl<'a> DeclarationValue<'a, CssMetadata> for $value<'a> {
			fn is_unknown(&self) -> bool {
				matches!(self, Self::Unknown(_))
			}

			fn needs_computing(&self) -> bool {
				matches!(self, Self::Unknown(_))
			}

			fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, _: Cursor) -> ParserResult<Self>
			where
				I: Iterator<Item = Cursor> + Clone,
			{
				Ok(Self::Index(p.parse::<$index<'a>>()?))
			}

			fn parse_unknown_declaration_value<I>(p: &mut Parser<'a, I>, _: Cursor) -> ParserResult<Self>
			where
				I: Iterator<Item = Cursor> + Clone,
			{
				Ok(Self::Unknown(p.parse::<ComponentValues<'a>>()?))
			}
		}

		impl<'a> NodeWithMetadata<CssMetadata> for $value<'a> {
			fn metadata(&self) -> CssMetadata {
				match self {
					Self::Index(index) => index.metadata(),
					Self::Unknown(_) => CssMetadata::default(),
				}
			}
		}
	};
}

font_feature_value!(FontFeatureValue, FontFeatureIndex);
font_feature_value!(FontFeatureValuePair, FontFeatureIndexPair);
font_feature_value!(FontFeatureValueList, FontFeatureIndexList);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontFeatureValuesRule, "@font-feature-values Taisho Gothic{}");
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values Otaru Kisa{@annotation{circled:1;black-boxed:3}}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values \"Otaru Kisa\",Bongo{font-display:swap;@swash{ornate:1}}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values Mars Serif{@styleset{alt-g:1;code:4 5}}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values MM Greek{@character-variant{alpha-2:1 2;gamma:12}}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values Bongo{@stylistic{foo:1}@historical-forms{bar:2}@ornaments{baz:3}}"
		);
		// An unknown at-rule within the block is invalid and ignored, but does not invalidate the rule.
		assert_parse!(
			CssAtomSet::ATOMS,
			FontFeatureValuesRule,
			"@font-feature-values Bongo{@swash{ornate:1}@nope{x:1}}"
		);
		// A syntax error in a font feature value declaration does not invalidate the block.
		assert_parse!(CssAtomSet::ATOMS, FontFeatureValuesRule, "@font-feature-values Bongo{@swash{swishy:3 5}}");
	}
}
