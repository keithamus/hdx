use crate::{properties::Property, selector::SelectorList};
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	syntax::BadDeclaration, Block, CursorSink, Parse, Parser, QualifiedRule, Result as ParserResult, State, ToCursors,
	T,
};
use hdx_proc_macro::visit;

use super::{rules, UnknownAtRule, UnknownQualifiedRule, Visit, Visitable};

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylerule"))]
#[visit]
pub struct StyleRule<'a> {
	pub selectors: SelectorList<'a>,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub style: StyleDeclaration<'a>,
}

impl<'a> Parse<'a> for StyleRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (selectors, style) = Self::parse_qualified_rule(p)?;
		Ok(Self { selectors, style })
	}
}

impl<'a> QualifiedRule<'a> for StyleRule<'a> {
	type Block = StyleDeclaration<'a>;
	type Prelude = SelectorList<'a>;
	type BadDeclaration = BadDeclaration<'a>;
}

impl ToCursors for StyleRule<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.selectors, s);
		ToCursors::to_cursors(&self.style, s);
	}
}

impl<'a> Visitable<'a> for StyleRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_style_rule(self);
		Visitable::accept(&self.selectors, v);
		Visitable::accept(&self.style, v);
	}
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "style-declaration"))]
#[visit]
pub struct StyleDeclaration<'a> {
	pub open: T!['{'],
	pub declarations: Vec<'a, (Property<'a>, Option<T![;]>)>,
	pub rules: Vec<'a, NestedGroupRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for StyleDeclaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, declarations, rules, close) = Self::parse_block(p)?;
		Ok(Self { open, declarations, rules, close })
	}
}

impl<'a> Block<'a> for StyleDeclaration<'a> {
	type Declaration = Property<'a>;
	type Rule = NestedGroupRule<'a>;
}

impl<'a> ToCursors for StyleDeclaration<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open, s);
		for (declaration, semicolon) in &self.declarations {
			ToCursors::to_cursors(declaration, s);
			if let Some(semicolon) = semicolon {
				s.append(semicolon.into());
			}
		}
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = &self.close {
			ToCursors::to_cursors(close, s);
		}
	}
}

impl<'a> Visitable<'a> for StyleDeclaration<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_style_declaration(self);
		for (declaration, _) in &self.declarations {
			Visitable::accept(declaration, v);
		}
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

// https://drafts.csswg.org/css-nesting/#conditionals
macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			ContainerRule<'a>: "container",
			LayerRule<'a>: "layer",
			MediaRule<'a>: "media",
			ScopeRule: "scope",
			SupportsRule<'a>: "supports",
		}
	};
}

macro_rules! nested_group_rule {
    ( $(
        $name: ident$(<$a: lifetime>)?: $str: pat,
    )+ ) => {
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum NestedGroupRule<'a> {
			$(
				$name(rules::$name$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>),
			BadDeclaration(BadDeclaration<'a>),
		}
	}
}
apply_rules!(nested_group_rule);

impl<'a> Parse<'a> for NestedGroupRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		if p.peek::<T![AtKeyword]>() {
			let c: Cursor = p.peek_n(1);
			macro_rules! parse_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $str: pat,
				)+ ) => {
					match p.parse_str_lower(c) {
						$($str => p.parse::<rules::$name>().map(Self::$name),)+
						_ => {
							let rule = p.parse::<UnknownAtRule>()?;
							Ok(Self::UnknownAt(rule))
						}
					}
				}
			}
			if let Ok(rule) = apply_rules!(parse_rule) {
				Ok(rule)
			} else {
				p.rewind(checkpoint);
				p.parse::<UnknownAtRule>().map(Self::UnknownAt)
			}
		} else if let Ok(rule) = p.parse::<StyleRule>() {
			Ok(Self::Style(rule))
		} else {
			p.rewind(checkpoint);
			if let Ok(rule) = p.parse::<UnknownQualifiedRule>() {
				Ok(Self::Unknown(rule))
			} else {
				p.rewind(checkpoint);
				let state = p.set_state(State::Nested);
				let declaration = p.parse::<BadDeclaration>();
				p.set_state(state);
				Ok(Self::BadDeclaration(declaration?))
			}
		}
	}
}

impl<'a> ToCursors for NestedGroupRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		macro_rules! match_rule {
			( $(
				$name: ident$(<$a: lifetime>)?: $str: pat,
			)+ ) => {
				match self {
					$(Self::$name(r) => ToCursors::to_cursors(r, s),)+
					Self::UnknownAt(r) => ToCursors::to_cursors(r, s),
					Self::Style(r) => ToCursors::to_cursors(r, s),
					Self::Unknown(r) => ToCursors::to_cursors(r, s),
					Self::BadDeclaration(r) => ToCursors::to_cursors(r, s),
				}
			}
		}
		apply_rules!(match_rule);
	}
}

impl<'a> Visitable<'a> for NestedGroupRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		macro_rules! match_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $ststr: pat,
				)+ ) => {
					match self {
						$(Self::$name(r) => Visitable::accept(r, v),)+
						Self::UnknownAt(r) => Visitable::accept(r, v),
						Self::Style(r) => Visitable::accept(r, v),
						Self::Unknown(r) => Visitable::accept(r, v),
						Self::BadDeclaration(_) => {},
					};
				}
			}
		apply_rules!(match_rule);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StyleRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleRule, "body{}");
		assert_parse!(StyleRule, "body,body{}");
		assert_parse!(StyleRule, "body{width:1px;}");
		assert_parse!(StyleRule, "body{opacity:0;}");
		assert_parse!(StyleRule, ".foo *{}", ".foo *{}");
		assert_parse!(StyleRule, ":nth-child(1){opacity:0;}");
		assert_parse!(StyleRule, ".foo{--bar:(baz);}");
		assert_parse!(StyleRule, ".foo{width: calc(1px + (var(--foo)) + 1px);}");
		assert_parse!(StyleRule, ".foo{--bar:1}");
		assert_parse!(StyleRule, ":root{--custom:{width:0;height:0;};}");
		// Semicolons are "allowed" in geneirc preludes
		assert_parse!(StyleRule, ":root{a;b{}}");
		// Bad Declarations should be parsable.
		assert_parse!(StyleRule, ":root{$(var)-size: 100%;}");
	}
}
