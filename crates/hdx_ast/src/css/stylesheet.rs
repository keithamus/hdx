use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	CursorStream, Parse, Parser, Result as ParserResult, StyleSheet as StyleSheetTrait, ToCursors, Vec, T,
};

use crate::{
	css::{rules, stylerule::StyleRule},
	syntax::{AtRule, QualifiedRule},
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylesheet"))]
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

impl<'a> ToCursors<'a> for StyleSheet<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
	}
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Charset: atom!("charset"),
			ColorProfile: atom!("color-profile"),
			Container: atom!("container"),
			CounterStyle: atom!("counter-style"),
			FontFace<'a>: atom!("font-face"),
			FontFeatureValues: atom!("font-feature-values"),
			FontPaletteValues: atom!("font-palette-values"),
			Import: atom!("import"),
			Keyframes<'a>: atom!("keyframes"),
			Layer: atom!("layer"),
			Media<'a>: atom!("media"),
			Namespace: atom!("namespace"),
			Page<'a>: atom!("page"),
			Property: atom!("property"),
			Scope: atom!("scope"),
			StartingStyle: atom!("starting-style"),
			Supports<'a>: atom!("supports"),

			// Deprecated Rules
			Document: atom!("document"),

			// Vendor Prefixed
			WebkitKeyframes<'a>: atom!("-webkit-keyframes"),

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocument: atom!("-moz-document"),
		}
	};
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
			UnknownAt(AtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(QualifiedRule<'a>)
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
							let rule = p.parse::<AtRule>()?;
							Ok(Self::UnknownAt(rule))
						}
					}
				}
			}
			if let Ok(rule) = apply_rules!(parse_rule) {
				Ok(rule)
			} else {
				p.rewind(checkpoint);
				p.parse::<AtRule>().map(Self::UnknownAt)
			}
		} else if let Ok(rule) = p.parse::<StyleRule>() {
			Ok(Self::Style(rule))
		} else {
			p.rewind(checkpoint);
			p.parse::<QualifiedRule>().map(Self::Unknown)
		}
	}
}

impl<'a> ToCursors<'a> for Rule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset(T![AtKeyword]), // atom!("charset")
	Page(T![AtKeyword]),    // atom!("page")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StyleSheet, 32);
		assert_size!(Rule, 456);
		assert_size!(AtRuleId, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleSheet, "body{}");
		assert_parse!(StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(StyleSheet, "body{width:1px;}");
		assert_parse!(StyleSheet, "body{width:1px;}.a{width:2px;}");
	}
}
