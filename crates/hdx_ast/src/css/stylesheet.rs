use hdx_atom::atom;
use hdx_derive::Atomizable;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, discard, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::{
		stylerule::StyleRule,
		rules::{PageRule, CharsetRule},
		unknown::{UnknownAtRule, UnknownRule},
	},
	Box, Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-stylesheet-contents
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		// 5.5.1. Consume a stylesheet’s contents
		// "To consume a stylesheet’s contents from a token stream input:"

		let span = parser.span();

		// "Let rules be an initially empty list of rules."
		let mut rules = parser.new_vec();
		loop {
			discard!(parser, Token::Comment(_) | Token::Whitespace | Token::Cdc | Token::Cdo);
			match parser.cur() {
				// Eof is the end of a StyleSheet
				Token::Eof => break,
				// "Consume an at-rule from input. If anything is returned, append it to rules."
				Token::AtKeyword(atom) => {
					let rule = match atom.to_ascii_lowercase() {
						atom!("charset") => {
							let rule = CharsetRule::parse(parser)?;
							Rule::Charset(parser.boxup(rule))
						}
						atom!("page") => {
							let rule = PageRule::parse(parser)?;
							Rule::Page(parser.boxup(rule))
						}
						_ => {
							let rule = UnknownAtRule::parse(parser)?;
							parser.warn(diagnostics::UnknownRule(rule.span).into());
							Rule::UnknownAt(parser.boxup(rule))
						}
					};
					rules.push(rule);
				}
				// "Consume a qualified rule from input. If anything is returned, append it to rules."
				_ => {
					let checkpoint = parser.checkpoint();
					let rule = match StyleRule::parse(parser) {
						Ok(rule) => Rule::Style(parser.boxup(rule)),
						Err(err) => {
							parser.rewind(checkpoint);
							parser.warn(err);
							let rule = UnknownRule::parse(parser)?;
							Rule::Unknown(parser.boxup(rule))
						}
					};
					rules.push(rule);
				}
			}
		}
		Ok(Self { rules }.spanned(span.end(parser.pos())))
	}
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
	UnknownAt(Box<'a, Spanned<UnknownAtRule<'a>>>),
	Unknown(Box<'a, Spanned<UnknownRule<'a>>>),
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

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<StyleSheet>(), 32);
		assert_eq!(size_of::<Rule>(), 16);
		assert_eq!(size_of::<AtRuleId>(), 1);
	}
}
