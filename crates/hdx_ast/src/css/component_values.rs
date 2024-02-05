use hdx_atom::Atom;
use hdx_lexer::{PairWise, Token};
use hdx_parser::{unexpected, Box, Parse, Parser, Result as ParserResult, Spanned, State, Vec};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

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
				parser.advance();
				Ok(Self::Token(token).spanned(span))
			}
		}
	}
}

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
				Token::RightCurly => {
					parser.advance();
				}
				c => values.push(ComponentValue::parse(parser)?),
			}
		}
		Ok(Self(values).spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for ComponentValues<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		todo!();
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
	pub value: Box<'a, Spanned<ComponentValues<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		if let Some(pairwise) = parser.cur().to_pairwise() {
			let ending_token = pairwise.end();
			let span = parser.span();
			let value = ComponentValues::parse(parser)?;
			Ok(Self { value: parser.boxup(value), pairwise }.spanned(span.end(parser.pos())))
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
		self.value.write_css(sink)?;
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
pub struct Function<'a> {
	pub name: Atom,
	pub value: Box<'a, Spanned<ComponentValues<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Function(name) => {
				let value = ComponentValues::parse(parser)?;
				Ok(Self { name, value: parser.boxup(value) }.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Function<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char('(')?;
		self.value.write_css(sink)?;
		sink.write_char(')')
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ComponentValue>(), 32);
		assert_eq!(size_of::<SimpleBlock>(), 16);
		assert_eq!(size_of::<Function>(), 16);
	}
}
