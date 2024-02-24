use hdx_atom::atom;
use hdx_derive::Atomizable;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, StyleSheet as StyleSheetTrait};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::{
		rules::{CharsetRule, PageRule},
		stylerule::StyleRule,
	},
	syntax::{AtRule, QualifiedRule},
	Box, Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		Ok(Self { rules: Self::parse_stylesheet(parser)? }.spanned(span.end(parser.pos())))
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

// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Rule<'a> {
	Charset(Box<'a, Spanned<CharsetRule>>),
	Page(Box<'a, Spanned<PageRule<'a>>>),
	Style(Box<'a, Spanned<StyleRule<'a>>>),
	UnknownAt(Box<'a, Spanned<AtRule<'a>>>),
	Unknown(Box<'a, Spanned<QualifiedRule<'a>>>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		Ok(match parser.cur() {
			Token::AtKeyword(atom) => match atom.to_ascii_lowercase() {
				atom!("charset") => {
					let rule = CharsetRule::parse(parser)?;
					Rule::Charset(parser.boxup(rule))
				}
				atom!("page") => {
					let rule = PageRule::parse(parser)?;
					Rule::Page(parser.boxup(rule))
				}
				_ => {
					let rule = AtRule::parse(parser)?;
					parser.warn(diagnostics::UnknownRule(rule.span).into());
					Rule::UnknownAt(parser.boxup(rule))
				}
			},
			// "Consume a qualified rule from input. If anything is returned, append it to rules."
			_ => {
				let checkpoint = parser.checkpoint();
				match StyleRule::parse(parser) {
					Ok(rule) => Rule::Style(parser.boxup(rule)),
					Err(err) => {
						parser.rewind(checkpoint);
						parser.warn(err);
						let rule = QualifiedRule::parse(parser)?;
						Rule::Unknown(parser.boxup(rule))
					}
				}
			}
		}
		.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for Rule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Style(rule) => rule.write_css(sink),
			Self::Charset(rule) => rule.write_css(sink),
			Self::Page(rule) => rule.write_css(sink),
			Self::UnknownAt(rule) => rule.write_css(sink),
			Self::Unknown(rule) => rule.write_css(sink),
		}
	}
}

#[derive(Atomizable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset, // atom!("charset")
	Page,    // atom!("page")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<StyleSheet>(), 32);
		assert_eq!(size_of::<Rule>(), 16);
		assert_eq!(size_of::<AtRuleId>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<StyleSheet>(&allocator, "body {}", "body{}");
		test_write::<StyleSheet>(&allocator, "body, body {}", "body,body{}");
		test_write::<StyleSheet>(&allocator, "body { width: 1px }", "body{width:1px}");
	}
}
