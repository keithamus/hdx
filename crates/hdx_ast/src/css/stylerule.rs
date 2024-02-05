use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, QualifiedRule, Result as ParserResult, expect};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::{properties::StyleProperty, selector::Selector},
	Box, Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StyleRule<'a> {
	pub selectors: Box<'a, Spanned<Selector<'a>>>,
	pub style: Box<'a, Spanned<StyleDeclaration<'a>>>,
}

impl<'a> Parse<'a> for StyleRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let (selectors, style) = Self::parse_qualified_rule(parser)?;
		Ok(Self { selectors: parser.boxup(selectors), style: parser.boxup(style) }.spanned(span.end(parser.pos())))
	}
}

impl<'a> QualifiedRule<'a> for StyleRule<'a> {
	type Block = StyleDeclaration<'a>;
	type Prelude = Selector<'a>;
}

impl<'a> WriteCss<'a> for StyleRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.selectors.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		self.style.write_css(sink)?;
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StyleDeclaration<'a> {
	pub declarations: Vec<'a, Spanned<StyleProperty<'a>>>,
	pub rules: Vec<'a, Spanned<StyleRule<'a>>>,
}

impl<'a> Parse<'a> for StyleDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		expect!(parser, Token::LeftCurly);
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::RightCurly => break,
				t @ Token::Eof => unexpected!(parser, t),
				_ => {
					let checkpoint = parser.checkpoint();
					if let Ok(decl) = StyleProperty::parse(parser) {
						declarations.push(decl)
					} else {
						parser.rewind(checkpoint);
						rules.push(StyleRule::parse(parser)?);
					}
				}
			}
		}
		Ok(Self { declarations, rules }.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for StyleDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.declarations.iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<StyleRule>(), 16);
	}
}
