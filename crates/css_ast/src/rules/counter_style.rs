use super::prelude::*;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};
use crate::{Inherits, PropertyGroup};
use csskit_derives::*;
use csskit_proc_macro::{node, syntax};

/// <https://drafts.csswg.org/css-counter-styles-3/#the-counter-style-rule>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = CounterStyle, property_kinds = Name)]
pub struct CounterStyleRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::CounterStyle)]
	pub name: T![AtKeyword],
	pub prelude: CounterStyleName,
	#[metadata(block)]
	pub block: CounterStyleRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for CounterStyleRule<'a> {
	const NODE_ID: NodeId = NodeId::CounterStyleRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.into()),
			_ => None,
		}
	}
}

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CounterStyleName(T![Ident]);

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CounterStyleRuleBlock<'a>(pub DeclarationList<'a, CounterStyleRuleStyleValue<'a>, CssMetadata>);

#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Declaration, property_kinds = Name)]
pub enum CounterStyleRuleStyleValue<'a> {
	Unknown(ComponentValues<'a>),
	System(SystemStyleValue<'a>),
	Symbols(SymbolsStyleValue<'a>),
	AdditiveSymbols(AdditiveSymbolsStyleValue<'a>),
	Negative(NegativeStyleValue<'a>),
	Prefix(PrefixStyleValue<'a>),
	Suffix(SuffixStyleValue<'a>),
	Range(RangeStyleValue<'a>),
	Pad(PadStyleValue<'a>),
	SpeakAs(SpeakAsStyleValue<'a>),
	Fallback(FallbackStyleValue),
}

impl<'a> DeclarationValue<'a, CssMetadata> for CounterStyleRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::InitialValue | CssAtomSet::Inherits | CssAtomSet::Syntax)
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
			CssAtomSet::System => Self::System(p.parse::<SystemStyleValue<'a>>()?),
			CssAtomSet::Symbols => Self::Symbols(p.parse::<SymbolsStyleValue<'a>>()?),
			CssAtomSet::AdditiveSymbols => Self::AdditiveSymbols(p.parse::<AdditiveSymbolsStyleValue<'a>>()?),
			CssAtomSet::Negative => Self::Negative(p.parse::<NegativeStyleValue<'a>>()?),
			CssAtomSet::Prefix => Self::Prefix(p.parse::<PrefixStyleValue<'a>>()?),
			CssAtomSet::Suffix => Self::Suffix(p.parse::<SuffixStyleValue<'a>>()?),
			CssAtomSet::Range => Self::Range(p.parse::<RangeStyleValue<'a>>()?),
			CssAtomSet::Pad => Self::Pad(p.parse::<PadStyleValue<'a>>()?),
			CssAtomSet::SpeakAs => Self::SpeakAs(p.parse::<SpeakAsStyleValue>()?),
			CssAtomSet::Fallback => Self::Fallback(p.parse::<FallbackStyleValue>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

#[syntax("	[ <integer [0,∞]> && <symbol> ]# ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AdditiveSymbolsStyleValue<'a>;

#[node]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FallbackStyleValue(pub CounterStyleName);

#[syntax(" <symbol> <symbol>? ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NegativeStyleValue<'a>;

#[syntax(" <integer [0,∞]> && <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PadStyleValue<'a>;

#[syntax(" <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PrefixStyleValue<'a>;

#[syntax(" [ [ <integer> | infinite ]{2} ]# | auto ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RangeStyleValue<'a>;

#[syntax(" auto | bullets | numbers | words | spell-out | <counter-style-name> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SpeakAsStyleValue<'a> {}

#[syntax(" <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SuffixStyleValue<'a>;

#[syntax(" <symbol>+ ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SymbolsStyleValue<'a>;

#[syntax(
	" cyclic | numeric | alphabetic | symbolic | additive | [fixed <integer>?] | [ extends <counter-style-name> ] "
)]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SystemStyleValue<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CounterStyleRule, "@counter-style thumbs {}");
	}
}
