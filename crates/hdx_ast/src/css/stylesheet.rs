use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{CursorSink, Parse, Parser, Result as ParserResult, StyleSheet as StyleSheetTrait, ToCursors, Vec, T};
use hdx_proc_macro::visit;

use crate::{
	css::{rules, stylerule::StyleRule, Visit, Visitable},
	syntax::{AtRule, QualifiedRule},
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylesheet"))]
#[visit]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self { rules: Self::parse_stylesheet(p)? })
	}
}

impl<'a> StyleSheetTrait<'a> for StyleSheet<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for StyleSheet<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
	}
}

impl<'a> Visitable<'a> for StyleSheet<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for rule in &self.rules {
			v.visit_style_sheet(self);
			Visitable::accept(rule, v);
		}
	}
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			CharsetRule: atom!("charset"),
			ColorProfileRule: atom!("color-profile"),
			ContainerRule<'a>: atom!("container"),
			CounterStyleRule: atom!("counter-style"),
			FontFaceRule<'a>: atom!("font-face"),
			FontFeatureValuesRule: atom!("font-feature-values"),
			FontPaletteValuesRule: atom!("font-palette-values"),
			ImportRule: atom!("import"),
			KeyframesRule<'a>: atom!("keyframes"),
			LayerRule<'a>: atom!("layer"),
			MediaRule<'a>: atom!("media"),
			NamespaceRule: atom!("namespace"),
			PageRule<'a>: atom!("page"),
			PropertyRule<'a>: atom!("property"),
			ScopeRule: atom!("scope"),
			StartingStyleRule: atom!("starting-style"),
			SupportsRule<'a>: atom!("supports"),

			// Deprecated Rules
			DocumentRule<'a>: atom!("document"),

			// Vendor Prefixed
			WebkitKeyframesRule<'a>: atom!("-webkit-keyframes"),

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocumentRule<'a>: atom!("-moz-document"),
		}
	};
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct UnknownAtRule<'a>(AtRule<'a>);

impl<'a> Parse<'a> for UnknownAtRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(p.parse::<AtRule>()?))
	}
}

impl<'a> ToCursors for UnknownAtRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
	}
}

impl<'a> Visitable<'a> for UnknownAtRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_unknown_at_rule(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct UnknownQualifiedRule<'a>(QualifiedRule<'a>);

impl<'a> Parse<'a> for UnknownQualifiedRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(p.parse::<QualifiedRule>()?))
	}
}

impl<'a> ToCursors for UnknownQualifiedRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
	}
}

impl<'a> Visitable<'a> for UnknownQualifiedRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_unknown_qualified_rule(self);
	}
}

macro_rules! rule {
    ( $(
        $name: ident$(<$a: lifetime>)?: $atom: pat,
    )+ ) => {
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum Rule<'a> {
			$(
				$name(rules::$name$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>)
		}
	}
}

apply_rules!(rule);

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		if p.peek::<T![AtKeyword]>() {
			let c: Cursor = p.peek_n(1);
			macro_rules! parse_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $atom: pat,
				)+ ) => {
					match p.parse_atom_lower(c) {
						$($atom => p.parse::<rules::$name>().map(Self::$name),)+
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
			p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
		}
	}
}

impl<'a> ToCursors for Rule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		macro_rules! match_rule {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match self {
					$(Self::$name(r) => ToCursors::to_cursors(r, s),)+
					Self::UnknownAt(r) => ToCursors::to_cursors(r, s),
					Self::Style(r) => ToCursors::to_cursors(r, s),
					Self::Unknown(r) => ToCursors::to_cursors(r, s),
				}
			}
		}
		apply_rules!(match_rule);
	}
}

impl<'a> Visitable<'a> for Rule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		macro_rules! match_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $atom: pat,
				)+ ) => {
					match self {
						$(Self::$name(r) => Visitable::accept(r, v),)+
						Self::UnknownAt(r) => Visitable::accept(r, v),
						Self::Style(r) => Visitable::accept(r, v),
						Self::Unknown(r) => Visitable::accept(r, v),
					};
				}
			}
		apply_rules!(match_rule);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StyleSheet, 32);
		assert_size!(Rule, 512);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleSheet, "body{}");
		assert_parse!(StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(StyleSheet, "body{width:1px;}");
		assert_parse!(StyleSheet, "body{width:1px;}.a{width:2px;}");
	}
}
