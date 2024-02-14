use hdx_lexer::Token;
use hdx_parser::{unexpected, AtRule, Parse, Parser, QualifiedRule, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::component_values::{ComponentValues, SimpleBlock};
use crate::{Atom, Box, Spanned};

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownAtRule<'a> {
	pub name: Atom,
	pub prelude: Box<'a, Option<Spanned<ComponentValues<'a>>>>,
	pub block: Box<'a, Option<Spanned<SimpleBlock<'a>>>>,
}

impl<'a> Parse<'a> for UnknownAtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::AtKeyword(name) => {
				let (prelude, block) = Self::parse_at_rule(parser)?;
				Ok(Self { name, prelude: parser.boxup(prelude), block: parser.boxup(block) }.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> AtRule<'a> for UnknownAtRule<'a> {
	type Block = SimpleBlock<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for UnknownAtRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@")?;
		sink.write_str(self.name.as_ref())?;
		if let Some(prelude) = &self.prelude.0 {
			prelude.write_css(sink)?;
			sink.write_trivia_char(' ')?;
		}
		self.block.write_css(sink)?;
		Ok(())
	}
}
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownRule<'a> {
	pub prelude: Box<'a, Spanned<ComponentValues<'a>>>,
	pub block: Box<'a, Spanned<SimpleBlock<'a>>>,
}

impl<'a> Parse<'a> for UnknownRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let (prelude, block) = Self::parse_qualified_rule(parser)?;
		Ok(Self { prelude: parser.boxup(prelude), block: parser.boxup(block) }.spanned(span.end(parser.pos())))
	}
}

impl<'a> QualifiedRule<'a> for UnknownRule<'a> {
	type Block = SimpleBlock<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for UnknownRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.prelude.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<UnknownAtRule>(), 24);
		assert_eq!(size_of::<UnknownRule>(), 16);
	}
}
