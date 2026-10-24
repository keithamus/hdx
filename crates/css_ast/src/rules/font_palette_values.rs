use super::prelude::*;
use crate::PaletteIdentifier;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};
use csskit_proc_macro::syntax;

/// Represents the `@font-palette-values` at-rule, e.g. `@font-palette-values --cooler { base-palette: 2 }`.
///
/// ```md
/// <font-palette-values-rule>
///  │├─ "@font-palette-values" ─ <dashed-ident> ─ <block> ─┤│
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#at-ruledef-font-palette-values>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.font-palette-values")
)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = FontPaletteValues, property_kinds = Name)]
pub struct FontPaletteValuesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::FontPaletteValues)]
	pub name: T![AtKeyword],
	pub prelude: PaletteIdentifier,
	#[metadata(block)]
	pub block: FontPaletteValuesRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for FontPaletteValuesRule<'a> {
	const NODE_ID: NodeId = NodeId::FontPaletteValuesRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.into()),
			_ => None,
		}
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontPaletteValuesRuleBlock<'a>(pub DeclarationList<'a, FontPaletteValuesRuleStyleValue<'a>, CssMetadata>);

/// The descriptors allowed inside a [`FontPaletteValuesRule`] block.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontPaletteValuesRuleStyleValue<'a> {
	FontFamily(FontPaletteValuesFontFamilyValue<'a>),
	BasePalette(BasePaletteValue<'a>),
	OverrideColors(OverrideColorsValue<'a>),
	Unknown(ComponentValues<'a>),
}

impl<'a> DeclarationValue<'a, CssMetadata> for FontPaletteValuesRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(
			p.to_atom::<CssAtomSet>(c),
			CssAtomSet::FontFamily | CssAtomSet::BasePalette | CssAtomSet::OverrideColors
		)
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
			CssAtomSet::FontFamily => Self::FontFamily(p.parse::<FontPaletteValuesFontFamilyValue<'a>>()?),
			CssAtomSet::BasePalette => Self::BasePalette(p.parse::<BasePaletteValue<'a>>()?),
			CssAtomSet::OverrideColors => Self::OverrideColors(p.parse::<OverrideColorsValue<'a>>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

/// ```text,ignore
/// <family-name>#
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#descdef-font-palette-values-font-family>
#[syntax(" <font-family-name># ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontPaletteValuesFontFamilyValue<'a>;

/// ```text,ignore
/// light | dark | <integer [0,∞]>
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#descdef-font-palette-values-base-palette>
#[syntax(" light | dark | <integer [0,∞]> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BasePaletteValue<'a> {}

/// ```text,ignore
/// [ <integer [0,∞]> <color> ]#
/// ```
///
/// <https://drafts.csswg.org/css-fonts/#descdef-font-palette-values-override-colors>
#[syntax(" [ <integer [0,∞]> <color> ]# ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OverrideColorsValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontPaletteValuesRule, "@font-palette-values --cooler{}");
		assert_parse!(CssAtomSet::ATOMS, FontPaletteValuesRule, "@font-palette-values --cooler{base-palette:dark}");
		assert_parse!(
			CssAtomSet::ATOMS,
			FontPaletteValuesRule,
			"@font-palette-values --cooler{font-family:Bixa;base-palette:1;override-colors:0 red,1 #0f0}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			FontPaletteValuesRule,
			"@font-palette-values --Alternate{font-family:\"Bungee Spice\",Bixa}"
		);
	}
}
