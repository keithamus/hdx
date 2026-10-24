use super::prelude::*;

/// <https://drafts.csswg.org/css-cascade-5/#layering>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.layer"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Layer)]
pub struct LayerRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Layer)]
	pub name: T![AtKeyword],
	#[metadata(prelude)]
	pub prelude: LayerNameList<'a>,
	pub block: Option<LayerRuleBlock<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub semicolon: Option<T![;]>,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LayerNameList<'a>(pub CommaSeparated<'a, LayerName<'a>, 0>);

#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LayerName<'a>(pub T![Ident], pub Vec<'a, (T![.], T![Ident])>);

impl<'a> Parse<'a> for LayerName<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut parts = Vec::new_in(p.alloc());
		let first = p.parse::<T![Ident]>()?;
		loop {
			if p.peek::<T![.]>() {
				let dot = p.parse::<T![.]>()?;
				let ident = p.parse::<T![Ident]>()?;
				parts.push((dot, ident));
			} else {
				return Ok(Self(first, parts));
			}
		}
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LayerRuleBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo{}");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo,bar;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo.bar,baz.bing.baz;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo.bar{body{color:black}}");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer{}");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer{body{color:black}}");
	}
}
