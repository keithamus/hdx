use hdx_atom::atom;
use hdx_derive::Atomizable;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, StyleSheet as StyleSheetTrait};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{
	css::{
		rules::{CharsetRule, PageRule, MediaRule, SupportsRule},
		stylerule::StyleRule,
	},
	syntax::{AtRule, QualifiedRule},
	Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self { rules: Self::parse_stylesheet(parser)? })
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
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Rule<'a> {
	Charset(CharsetRule),
	Page(PageRule<'a>),
	Style(StyleRule<'a>),
	Media(MediaRule<'a>),
	Supports(SupportsRule<'a>),
	UnknownAt(AtRule<'a>),
	Unknown(QualifiedRule<'a>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.cur() {
			Token::AtKeyword(atom) => match atom.to_ascii_lowercase() {
				atom!("charset") => Rule::Charset(CharsetRule::parse(parser)?),
				atom!("page") => Rule::Page(PageRule::parse(parser)?),
				atom!("media") => Rule::Media(MediaRule::parse(parser)?),
				atom!("media") => Rule::Media(MediaRule::parse(parser)?),
				atom!("supports") => Rule::Supports(SupportsRule::parse(parser)?),
				_ => {
					let span = parser.span();
					let rule = AtRule::parse(parser)?;
					parser.warn(diagnostics::UnknownRule(span.end(parser.pos())).into());
					Rule::UnknownAt(rule)
				}
			},
			// "Consume a qualified rule from input. If anything is returned, append it to rules."
			_ => {
				let checkpoint = parser.checkpoint();
				match StyleRule::parse(parser) {
					Ok(rule) => Rule::Style(rule),
					Err(err) => {
						parser.rewind(checkpoint);
						parser.warn(err);
						let rule = QualifiedRule::parse(parser)?;
						Rule::Unknown(rule)
					}
				}
			}
		})
	}
}

impl<'a> WriteCss<'a> for Rule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Style(rule) => rule.write_css(sink),
			Self::Charset(rule) => rule.write_css(sink),
			Self::Page(rule) => rule.write_css(sink),
			Self::Media(rule) => rule.write_css(sink),
			Self::Supports(rule) => rule.write_css(sink),
			Self::UnknownAt(rule) => rule.write_css(sink),
			Self::Unknown(rule) => rule.write_css(sink),
		}
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
		assert_size!(Rule, 144);
		assert_size!(AtRuleId, 1);
	}

	#[test]
	fn test_writes() {
		// assert_parse!(StyleSheet, "body{}");
		// assert_parse!(StyleSheet, "body,body{}");
		// assert_parse!(StyleSheet, "body{width:1px}");
	}
}
