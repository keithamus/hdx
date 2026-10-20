use crate::{
	CssAtomSet, CssMetadata, SelectorList, StyleValue, UnknownAtRule, UnknownQualifiedRule, diagnostics::CssDiagnostic,
	rules,
};
use css_parse::{
	Box, Cursor, DeclarationGroup, Diagnostic, Parse, Parser, QualifiedRule, Result as ParserResult, RuleVariants,
};
use csskit_derives::*;
use csskit_proc_macro::node;

/// Represents a "Style Rule", such as `body { width: 100% }`. See also the CSS-OM [CSSStyleRule][1] interface.
///
/// The Style Rule is comprised of two child nodes: the [SelectorList] represents the selectors of the rule.
/// Each [Declaration][css_parse::Declaration] will have a [StyleValue], and each rule will be a [NestedGroupRule].
///
/// [1]: https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = StyleRule)]
pub struct StyleRule<'a> {
	#[metadata(block)]
	pub rule: QualifiedRule<'a, SelectorList<'a>, StyleValue<'a>, NestedGroupRule<'a>, CssMetadata>,
}

// https://drafts.csswg.org/css-nesting/#conditionals
macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Container(ContainerRule<'a>): "container",
			Layer(LayerRule<'a>): "layer",
			Media(MediaRule<'a>): "media",
			Scope(ScopeRule<'a>): "scope",
		}
	};
}

macro_rules! nested_group_rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		/// <https://drafts.csswg.org/cssom-1/#the-cssrule-interface>
		#[node]
		#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		#[derive(csskit_derives::NodeWithMetadata)]
		pub enum NestedGroupRule<'a> {
			$(
				$name(Box<'a, rules::$ty$(<$a>)?>),
			)+
			Supports(Box<'a, rules::SupportsRule<'a>>),
			UnknownAt(Box<'a, UnknownAtRule<'a>>),
			Style(Box<'a, StyleRule<'a>>),
			Unknown(Box<'a, UnknownQualifiedRule<'a>>),
			Declarations(DeclarationGroup<'a, StyleValue<'a>, CssMetadata>),
		}
	}
}
apply_rules!(nested_group_rule);

impl<'a> RuleVariants<'a> for NestedGroupRule<'a> {
	type DeclarationValue = StyleValue<'a>;
	type Metadata = CssMetadata;

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
			)+ ) => {
				match p.to_atom::<CssAtomSet>(name) {
					$(CssAtomSet::$name => p.parse::<Box<rules::$ty>>().map(Self::$name),)+
					CssAtomSet::Supports => p.parse::<rules::SupportsRule>().map(|r| Self::Supports(Box::new_in(p.alloc(), r))),
					_ => Err(Diagnostic::new(name.into(), Diagnostic::unexpected_at_rule))?,
				}
			}
		}
		apply_rules!(parse_rule)
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<UnknownAtRule>>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<StyleRule>>().map(Self::Style)
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<UnknownQualifiedRule>>().map(Self::Unknown)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::UnknownAt(_) | Self::Unknown(_))
	}

	fn from_declaration_group(
		group: css_parse::DeclarationGroup<'a, Self::DeclarationValue, Self::Metadata>,
	) -> Option<Self> {
		Some(Self::Declarations(group))
	}
}

impl<'a> Parse<'a> for NestedGroupRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_span};

	#[cfg(feature = "visitable")]
	use crate::assert_visits;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body,body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{width:1px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{opacity:0;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo *{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":nth-child(1){opacity:0;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{--bar:(baz);}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{width: calc(1px + (var(--foo)) + 1px);}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{--bar:1}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{--custom:{width:0;height:0;};}");
		// Semicolons are "allowed" in geneirc preludes
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{a;b{}}");
		// Bad Declarations should be parsable.
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{$(var)-size: 100%;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".md{--:ra( ;86)}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "a{@supports (color:red){color:red;}}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "a{@supports selector(a){color:red;b{color:blue;}}}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "a{@container (width>0){color:red;}}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "a{@layer foo{color:red;}}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "a{@scope(.card) to (img){color:red;}}");
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			CssAtomSet::ATOMS,
			StyleRule,
			r#"
			body{} a{}
			^^^^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			StyleRule,
			r#"
			ul li:nth-of-type(2n) {} a{}
			^^^^^^^^^^^^^^^^^^^^^^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			StyleRule,
			r#"
			:nth-child(-2n+1){opacity:0;}
			^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
		"#
		);
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		assert_visits!(
			":root{html:has(&[open]){overflow:hidden}}",
			StyleRule,
			SelectorList,
			CompoundSelector,
			PseudoClass,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			HasPseudoFunction,
			SelectorList,
			CompoundSelector,
			Combinator,
			Attribute,
			StyleValue,
			OverflowStyleValue,
			OverflowBlockStyleValue
		);
	}
}
