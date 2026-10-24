use super::prelude::*;
use crate::Percentage;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};
use css_parse::NoBlockAllowed;

/// <https://drafts.csswg.org/css-animations/#at-ruledef-keyframes>
#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.keyframes"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Keyframes, property_kinds = Name)]
pub struct KeyframesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Keyframes)]
	pub name: T![AtKeyword],
	pub prelude: KeyframesName,
	#[metadata(block)]
	pub block: KeyframesRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for KeyframesRule<'a> {
	const NODE_ID: NodeId = NodeId::KeyframesRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.into()),
			_ => None,
		}
	}
}

#[node]
#[derive(Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum KeyframesName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframesName {
	fn invalid_ident(atom: CssAtomSet) -> bool {
		matches!(atom, CssAtomSet::Default | CssAtomSet::Initial | CssAtomSet::Unset | CssAtomSet::None)
	}
}

// Must use Parse rather than Build so ReservedKeyframeName errors can be emitted
impl<'a> Parse<'a> for KeyframesName {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![String]>() {
			return Ok(Self::String(p.parse::<T![String]>()?));
		}
		let ident = p.parse::<T![Ident]>()?;
		if KeyframesName::invalid_ident(p.to_atom::<CssAtomSet>(ident.into())) {
			Err(Diagnostic::new(ident.into(), Diagnostic::reserved_keyframe_name))?
		}
		Ok(Self::Ident(ident))
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct KeyframesRuleBlock<'a>(pub RuleList<'a, Keyframe<'a>, CssMetadata>);

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Keyframe<'a>(
	#[metadata(block)]
	pub  QualifiedRule<
		'a,
		KeyframeSelectors<'a>,
		StyleValue<'a>,
		NoBlockAllowed<StyleValue<'a>, CssMetadata>,
		CssMetadata,
	>,
);

#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct KeyframeSelectors<'a>(pub CommaSeparated<'a, KeyframeSelector>);

#[node]
#[derive(
	Peek, Parse, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum KeyframeSelector {
	#[atom(CssAtomSet::From)]
	From(T![Ident]),
	#[atom(CssAtomSet::To)]
	To(T![Ident]),
	Percent(Percentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes foo{}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes\"include\"{}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{0%{rotate:0deg}100%{rotate:360deg}}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{from,0%{rotate:0deg}to,100%{rotate:360deg}}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{to{rotate:360deg}}");
		assert_parse!(
			CssAtomSet::ATOMS,
			KeyframesRule,
			"@keyframes x{to{animation-timing-function:cubic-bezier(0,0,0.2,1)}}"
		);
	}
}
