use hdx_atom::{atom, Atom};
use hdx_lexer::{PairWise, Token};
use hdx_parser::{
	expect, unexpected, AtRule as AtRuleTrait, Block as BlockTrait, Box, Parse, Parser,
	QualifiedRule as QualifiedRuleTrait, Result as ParserResult, Span, Spanned, State, Vec,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct ComponentValues<'a>(pub Vec<'a, Spanned<ComponentValue<'a>>>);

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let mut values = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Eof => break,
				Token::RightCurly if parser.is(State::Nested) => break,
				// ComponentValues can be passed a "stop token" which could be any token.
				// In reality it is only ever called with a comma-token or semicolon-token.
				Token::Semicolon if parser.is(State::StopOnSemicolon) => break,
				Token::Comma if parser.is(State::StopOnComma) => break,
				_ => values.push(ComponentValue::parse(parser)?),
			}
		}
		Ok(Self(values).spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for ComponentValues<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for value in &self.0 {
			value.write_css(sink)?;
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(Spanned<SimpleBlock<'a>>),
	Function(Spanned<Function<'a>>),
	Token(Token),
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::LeftCurly | Token::LeftSquare | Token::LeftParen => {
				Ok(Self::SimpleBlock(SimpleBlock::parse(parser)?).spanned(span.end(parser.pos())))
			}
			Token::Function(_) => Ok(Self::Function(Function::parse(parser)?).spanned(span.end(parser.pos()))),
			token => {
				parser.advance_including_whitespace();
				Ok(Self::Token(token).spanned(span))
			}
		}
	}
}

impl<'a> WriteCss<'a> for ComponentValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::SimpleBlock(b) => b.write_css(sink),
			Self::Function(f) => f.write_css(sink),
			Self::Token(token) => {
				match token {
					Token::Ident(name) => sink.write_str(name.as_ref())?,
					Token::AtKeyword(name) => {
						sink.write_char('@')?;
						sink.write_str(name.as_ref())?;
					}
					Token::Hash(hash) | Token::HashId(hash) => {
						sink.write_char('#')?;
						sink.write_str(hash.as_ref())?;
					}
					Token::String(string) => {
						sink.write_char('"')?;
						sink.write_str(string.as_ref())?;
						sink.write_char('"')?;
					}
					Token::Url(url) => {
						sink.write_str("url(")?;
						sink.write_str(url.as_ref())?;
						sink.write_str("\")")?;
					}
					Token::Delim(ch) => {
						sink.write_char(*ch)?;
					}
					Token::Number(n, _) => sink.write_str(&format!("{}", n))?,
					Token::Dimension(n, unit, _) => {
						sink.write_str(&format!("{}", n))?;
						sink.write_str(unit.as_ref())?;
					}
					Token::Whitespace => sink.write_char(' ')?,
					Token::Cdo => sink.write_str("<!--")?,
					Token::Cdc => sink.write_str("-->")?,
					Token::Colon => sink.write_char(':')?,
					Token::Semicolon => sink.write_char(';')?,
					Token::Comma => sink.write_char(',')?,
					Token::LeftSquare => sink.write_char('[')?,
					Token::RightSquare => sink.write_char(']')?,
					Token::LeftParen => sink.write_char('(')?,
					Token::RightParen => sink.write_char(')')?,
					Token::LeftCurly => sink.write_char('{')?,
					Token::RightCurly => sink.write_char('}')?,
					Token::Undetermined => {}
					Token::Comment(content) => sink.write_trivia_str(content.as_ref())?,
					Token::Function(name) => {
						sink.write_str(name.as_ref())?;
						sink.write_char('(')?;
					}
					Token::Eof | Token::BadString | Token::BadUrl => {}
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	pub pairwise: PairWise,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		if let Some(pairwise) = parser.cur().to_pairwise() {
			let span = parser.span();
			let mut values = parser.new_vec();
			let ending_token = pairwise.end();
			parser.advance();
			loop {
				match parser.cur() {
					Token::Eof => break,
					t if t == ending_token => break,
					_ => values.push(ComponentValue::parse(parser)?),
				}
			}
			if parser.cur() == pairwise.end() {
				parser.advance();
			} else {
				unexpected!(parser)
			}
			Ok(Self { values, pairwise }.spanned(span.end(parser.pos())))
		} else {
			unexpected!(parser)
		}
	}
}

impl<'a> WriteCss<'a> for SimpleBlock<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self.pairwise {
			PairWise::Square => sink.write_char('[')?,
			PairWise::Curly => sink.write_char('{')?,
			PairWise::Paren => sink.write_char('(')?,
		}
		for value in &self.values {
			value.write_css(sink)?;
		}
		match self.pairwise {
			PairWise::Square => sink.write_char(']')?,
			PairWise::Curly => sink.write_char('}')?,
			PairWise::Paren => sink.write_char(')')?,
		}
		Ok(())
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum Rule<'a> {
	AtRule(Spanned<AtRule<'a>>),
	QualifiedRule(Spanned<QualifiedRule<'a>>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		Ok(match parser.cur() {
			Token::AtKeyword(_) => Rule::AtRule(AtRule::parse(parser)?),
			_ => Rule::QualifiedRule(QualifiedRule::parse(parser)?),
		}
		.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for Rule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::AtRule(value) => value.write_css(sink),
			Self::QualifiedRule(value) => value.write_css(sink),
		}
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Block<'a> {
	pub declarations: Vec<'a, Spanned<Declaration<'a>>>,
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
}

impl<'a> Parse<'a> for Block<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let (declarations, rules) = Self::parse_block(parser)?;
		Ok(Self { declarations, rules }.spanned(span.end(parser.pos())))
	}
}

impl<'a> BlockTrait<'a> for Block<'a> {
	type Declaration = Declaration<'a>;
	type Rule = Rule<'a>;
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Declaration<'a> {
	pub name: Atom,
	pub value: Spanned<ComponentValues<'a>>,
	pub important: bool,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
impl<'a> Parse<'a> for Declaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Ident(name) => {
				parser.advance();
				expect!(parser, Token::Colon);
				parser.advance();
				parser.set(State::StopOnSemicolon);
				parser.set(State::Nested);
				let mut value = ComponentValues::parse(parser)?;
				parser.unset(State::StopOnSemicolon | State::StopOnComma);
				let mut iter = value.node.0.iter_mut();
				let important = matches!(
					iter.nth_back(1),
					Some(Spanned { node: ComponentValue::Token(Token::Ident(atom!("important"))), .. })
				) && matches!(
					iter.nth_back(2),
					Some(Spanned { node: ComponentValue::Token(Token::Delim('!')), .. })
				);
				Ok(Self { name, value, important }.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Declaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.name.write_css(sink)?;
		sink.write_char(':')?;
		sink.write_trivia_char(' ')?;
		self.value.write_css(sink)?;
		if self.important {
			sink.write_str(" !important")?;
		}
		sink.write_char(';')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Block<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		for decl in &self.declarations {
			decl.write_css(sink)?;
		}
		for rule in &self.rules {
			rule.write_css(sink)?;
		}
		sink.write_char('}')
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct AtRule<'a> {
	pub name: Atom,
	pub prelude: Box<'a, Spanned<ComponentValues<'a>>>,
	pub block: Box<'a, Spanned<Block<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::AtKeyword(name) => {
				let (prelude_opt, block_opt) = Self::parse_at_rule(parser)?;
				let prelude = prelude_opt.unwrap_or_else(|| ComponentValues(parser.new_vec()).spanned(Span::dummy()));
				let block = block_opt.unwrap_or_else(|| {
					Block { declarations: parser.new_vec(), rules: parser.new_vec() }.spanned(Span::dummy())
				});
				Ok(Self { name, prelude: parser.boxup(prelude), block: parser.boxup(block) }
					.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> AtRuleTrait<'a> for AtRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for AtRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@")?;
		sink.write_str(self.name.as_ref())?;
		self.prelude.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a> {
	pub prelude: Box<'a, Spanned<ComponentValues<'a>>>,
	pub block: Box<'a, Spanned<Block<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
impl<'a> Parse<'a> for QualifiedRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let (prelude, block) = Self::parse_qualified_rule(parser)?;
		Ok(Self { prelude: parser.boxup(prelude), block: parser.boxup(block) }.spanned(span.end(parser.pos())))
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for QualifiedRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.prelude.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: Atom,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		match parser.cur() {
			Token::Function(name) => {
				let span = parser.span();
				let mut values = parser.new_vec();
				parser.advance();
				loop {
					match parser.cur() {
						Token::Eof => break,
						Token::RightParen => break,
						_ => values.push(ComponentValue::parse(parser)?),
					}
				}
				expect!(parser, Token::RightParen);
				parser.advance();
				Ok(Self { name, values }.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Function<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char('(')?;
		for value in &self.values {
			value.write_css(sink)?;
		}
		sink.write_char(')')
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ComponentValues>(), 32);
		assert_eq!(size_of::<ComponentValue>(), 56);
		assert_eq!(size_of::<SimpleBlock>(), 40);
		assert_eq!(size_of::<Function>(), 40);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<ComponentValue>(&allocator, "foo", "foo");
		test_write::<SimpleBlock>(&allocator, "[foo]", "[foo]");
		test_write::<SimpleBlock>(&allocator, "(one two three)", "(one two three)");
		test_write::<SimpleBlock>(&allocator, "(one(two))", "(one(two))");
		test_write::<SimpleBlock>(&allocator, "{one(two)}", "{one(two)}");
		test_write::<SimpleBlock>(&allocator, "{}", "{}");
		test_write::<SimpleBlock>(&allocator, "{foo}", "{foo}");
		test_write::<SimpleBlock>(&allocator, "{foo:bar}", "{foo:bar}");
		test_write::<Function>(&allocator, "one((two) three)", "one((two)three)");
		test_write::<ComponentValues>(&allocator, "a b c d", "a b c d");
		test_write::<ComponentValues>(&allocator, "body { color: black }", "body {color: black }");
		test_write::<ComponentValues>(&allocator, "body ", "body ");
		test_write::<Block>(&allocator, "{}", "{}");
		test_write::<Block>(&allocator, "{foo:bar}", "{foo:bar;}");
		test_write::<Block>(&allocator, "{foo:bar;baz:bing}", "{foo:bar;baz:bing;}");
	}
}
