use hdx_atom::{atom, Atom};
use hdx_lexer::{PairWise, Token};
use hdx_parser::{
	expect, unexpected, AtRule as AtRuleTrait, Block as BlockTrait, Parse, Parser, QualifiedRule as QualifiedRuleTrait,
	Result as ParserResult, Span, Spanned, State, Vec,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct ComponentValues<'a>(pub Vec<'a, Spanned<ComponentValue<'a>>>);

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Eof => break,
				Token::RightCurly if parser.is(State::Nested) => break,
				// ComponentValues can be passed a "stop token" which could be any token.
				// In reality it is only ever called with a comma-token or semicolon-token.
				Token::Semicolon if parser.is(State::StopOnSemicolon) => break,
				Token::Comma if parser.is(State::StopOnComma) => break,
				_ => values.push(ComponentValue::parse_spanned(parser)?),
			}
		}
		Ok(Self(values))
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
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(SimpleBlock<'a>),
	Function(Function<'a>),
	Token(Token),
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::LeftCurly | Token::LeftSquare | Token::LeftParen => {
				Ok(Self::SimpleBlock(SimpleBlock::parse(parser)?))
			}
			Token::Function(_) => Ok(Self::Function(Function::parse(parser)?)),
			token => {
				parser.advance_including_whitespace();
				Ok(Self::Token(token))
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
					Token::Ident(name) => sink.write_str(name)?,
					Token::AtKeyword(name) => {
						sink.write_char('@')?;
						sink.write_str(name)?;
					}
					Token::Hash(hash) | Token::HashId(hash) => {
						sink.write_char('#')?;
						sink.write_str(hash)?;
					}
					Token::String(string, quote) => {
						sink.write_with_quotes(string, *quote, false)?;
					}
					Token::Url(url, quote) => {
						atom!("url").write_css(sink)?;
						sink.write_char('(')?;
						sink.write_with_quotes(url.as_ref(), *quote, true)?;
						sink.write_char(')')?;
					}
					Token::Delim(ch) => {
						sink.write_char(*ch)?;
					}
					Token::Number(n, _) => sink.write_str(&format!("{}", n))?,
					Token::Dimension(n, unit, _) => {
						sink.write_str(&format!("{}", n))?;
						sink.write_str(unit)?;
					}
					Token::Whitespace => sink.write_char(' ')?,
					Token::Cdo => atom!("<!--").write_css(sink)?,
					Token::Cdc => atom!("-->").write_css(sink)?,
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
					Token::Comment(content) => sink.write_comment(content)?,
					Token::Function(name) => {
						sink.write_str(name)?;
						sink.write_char('(')?;
					}
					Token::Eof | Token::BadString | Token::BadUrl => {}
				}
				Ok(())
			}
		}
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	pub pairwise: PairWise,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(pairwise) = parser.cur().to_pairwise() {
			let mut values = parser.new_vec();
			let ending_token = pairwise.end();
			parser.advance();
			loop {
				match parser.cur() {
					Token::Eof => break,
					t if t == ending_token => break,
					_ => values.push(ComponentValue::parse_spanned(parser)?),
				}
			}
			if parser.cur() == pairwise.end() {
				parser.advance();
			} else {
				unexpected!(parser)
			}
			Ok(Self { values, pairwise })
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

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum Rule<'a> {
	AtRule(AtRule<'a>),
	QualifiedRule(QualifiedRule<'a>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.cur() {
			Token::AtKeyword(_) => Rule::AtRule(AtRule::parse(parser)?),
			_ => Rule::QualifiedRule(QualifiedRule::parse(parser)?),
		})
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

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Block<'a> {
	pub declarations: Vec<'a, Spanned<Declaration<'a>>>,
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
}

impl<'a> Parse<'a> for Block<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (declarations, rules) = Self::parse_block(parser)?;
		Ok(Self { declarations, rules })
	}
}

impl<'a> BlockTrait<'a> for Block<'a> {
	type Declaration = Declaration<'a>;
	type Rule = Rule<'a>;
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Declaration<'a> {
	pub name: Atom,
	pub value: Spanned<ComponentValues<'a>>,
	pub important: bool,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
impl<'a> Parse<'a> for Declaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Ident(name) => {
				parser.advance();
				expect!(parser, Token::Colon);
				parser.advance();
				parser.set(State::StopOnSemicolon);
				parser.set(State::Nested);
				let mut value = ComponentValues::parse_spanned(parser)?;
				parser.unset(State::StopOnSemicolon | State::StopOnComma);
				let mut iter = value.node.0.iter_mut();
				let important = matches!(
					iter.nth_back(1),
					Some(Spanned { node: ComponentValue::Token(Token::Ident(atom!("important"))), .. })
				) && matches!(
					iter.nth_back(2),
					Some(Spanned { node: ComponentValue::Token(Token::Delim('!')), .. })
				);
				Ok(Self { name, value, important })
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Declaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.name.write_css(sink)?;
		sink.write_char(':')?;
		sink.write_whitespace()?;
		self.value.write_css(sink)?;
		if self.important {
			sink.write_whitespace()?;
			sink.write_char('!')?;
			atom!("important").write_css(sink)?;
		}
		sink.write_char(';')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Block<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		for decl in &self.declarations {
			sink.write_indent()?;
			decl.write_css(sink)?;
			sink.write_newline()?;
		}
		for rule in &self.rules {
			sink.write_indent()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_char('}')
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct AtRule<'a> {
	pub name: Atom,
	pub prelude: Spanned<ComponentValues<'a>>,
	pub block: Spanned<Block<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::AtKeyword(name) => {
				let (prelude_opt, block_opt) = Self::parse_at_rule(parser)?;
				let prelude = prelude_opt
					.unwrap_or_else(|| Spanned { node: ComponentValues(parser.new_vec()), span: Span::dummy() });
				let block = block_opt.unwrap_or_else(|| Spanned {
					node: Block { declarations: parser.new_vec(), rules: parser.new_vec() },
					span: Span::dummy(),
				});
				Ok(Self { name, prelude, block })
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
		sink.write_char('@')?;
		sink.write_str(self.name.as_ref())?;
		self.prelude.write_css(sink)?;
		sink.write_whitespace()?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a> {
	pub prelude: Spanned<ComponentValues<'a>>,
	pub block: Spanned<Block<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
impl<'a> Parse<'a> for QualifiedRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (prelude, block) = Self::parse_qualified_rule(parser)?;
		Ok(Self { prelude, block })
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for QualifiedRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.prelude.write_css(sink)?;
		sink.write_whitespace()?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: Atom,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Function(name) => {
				let mut values = parser.new_vec();
				parser.advance();
				loop {
					match parser.cur() {
						Token::Eof => break,
						Token::RightParen => break,
						_ => values.push(ComponentValue::parse_spanned(parser)?),
					}
				}
				expect!(parser, Token::RightParen);
				parser.advance();
				Ok(Self { name, values })
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
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ComponentValues, 32);
		assert_size!(ComponentValue, 48);
		assert_size!(SimpleBlock, 40);
		assert_size!(Function, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ComponentValue, "foo");
		assert_parse!(SimpleBlock, "[foo]");
		assert_parse!(SimpleBlock, "(one two three)");
		assert_parse!(SimpleBlock, "(one(two))");
		assert_parse!(SimpleBlock, "{one(two)}");
		assert_parse!(SimpleBlock, "{}");
		assert_parse!(SimpleBlock, "{foo}");
		assert_parse!(SimpleBlock, "{foo:bar}");
		assert_parse!(Function, "one((two)three)");
		assert_parse!(ComponentValues, "a b c d");
		assert_parse!(ComponentValues, "body {color: black }");
		assert_parse!(ComponentValues, "body ");
		assert_parse!(Block, "{\n}");
		assert_parse!(Block, "{\n\tfoo: bar;\n}");
		assert_parse!(Block, "{\n\tfoo: bar;\n\tbaz: bing;\n}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Block, "{\n\tfoo: bar;\n\tbaz:bing;\n}", "{foo:bar;baz:bing;}");
	}
}
