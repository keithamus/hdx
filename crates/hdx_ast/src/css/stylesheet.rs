use hdx_atom::atom;
use hdx_derive::{Atomizable, Visitable};
use hdx_parser::{Parse, Parser, Result as ParserResult, Spanned, StyleSheet as StyleSheetTrait, Vec, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{
	css::{rules, stylerule::StyleRule},
	syntax::{AtRule, QualifiedRule},
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Visitable, Debug, PartialEq, Hash)]
#[visitable(call)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylesheet"))]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
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

impl<'a> WriteCss<'a> for StyleSheet<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for rule in &self.rules {
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
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
		#[derive(Visitable, PartialEq, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum Rule<'a> {
			$(
				#[visitable(skip)]
				$name(rules::$name$(<$a>)?),
			)+
			#[visitable(skip, call = visit_unknown_at_rule)]
			UnknownAt(AtRule<'a>),
			Style(StyleRule<'a>),
			#[visitable(skip, call = visit_unknown_rule)]
			Unknown(QualifiedRule<'a>)
		}
	}
}

apply_rules!(rule);

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		if let Some(token) = p.peek::<T![AtKeyword]>() {
			macro_rules! parse_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $atom: pat,
				)+ ) => {
					match p.parse_atom_lower(token) {
						$($atom => p.parse::<rules::$name>().map(Self::$name),)+
						_ => {
							let rule = p.parse_spanned::<AtRule>()?;
							Ok(Self::UnknownAt(rule.node))
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

impl<'a> WriteCss<'a> for Rule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		macro_rules! write_css {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match self {
					Self::Unknown(v) => v.write_css(sink),
					Self::UnknownAt(v) => v.write_css(sink),
					Self::Style(v) => v.write_css(sink),
					$(
						Self::$name(v) => v.write_css(sink),
					)+
				}
			}
		}
		apply_rules!(write_css)
	}
}

#[derive(Atomizable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset, // atom!("charset")
	Page,    // atom!("page")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StyleSheet, 32);
		assert_size!(Rule, 344);
		assert_size!(AtRuleId, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleSheet, "body {\n}\n");
		assert_parse!(StyleSheet, "body, tr:nth-child(n-1) {\n}\n");
		assert_parse!(StyleSheet, "body {\n\twidth: 1px;\n}\n");
		assert_parse!(StyleSheet, "body {\n\twidth: 1px;\n}\n.a {\n\twidth: 2px;\n}\n");
	}

	#[test]
	fn test_minify() {
		assert_minify!(StyleSheet, "body {\n\twidth: 1px;\n}\n", "body{width:1px}");
	}
}
