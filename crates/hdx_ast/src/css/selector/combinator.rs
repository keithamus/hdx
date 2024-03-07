use hdx_lexer::Token;
use hdx_parser::{discard, expect, peek, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	Column,            // ||
}

impl<'a> Parse<'a> for Combinator {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if matches!(parser.cur(), Token::Whitespace)
			&& !peek!(parser, Token::Delim('>') | Token::Delim('+') | Token::Delim('~') | Token::Delim('|'))
		{
			loop {
				parser.advance_including_whitespace();
				if !matches!(parser.cur(), Token::Whitespace) {
					break;
				}
			}
			return Ok(Self::Descendant);
		}
		discard!(parser, Token::Whitespace);
		match parser.cur() {
			Token::Delim(c) => match c {
				'>' => {
					parser.advance();
					Ok(Self::Child)
				}
				'+' => {
					parser.advance();
					Ok(Self::NextSibling)
				}
				'~' => {
					parser.advance();
					Ok(Self::SubsequentSibling)
				}
				'|' => {
					parser.advance_including_whitespace();
					expect!(parser, Token::Delim('|'));
					parser.advance();
					Ok(Self::Column)
				}
				_ => unexpected!(parser),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Combinator {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Descendant => sink.write_char(' '),
			Self::Child => {
				sink.write_whitespace()?;
				sink.write_char('>')?;
				sink.write_whitespace()
			}
			Self::NextSibling => {
				sink.write_whitespace()?;
				sink.write_char('+')?;
				sink.write_whitespace()
			}
			Self::SubsequentSibling => {
				sink.write_whitespace()?;
				sink.write_char('~')?;
				sink.write_whitespace()
			}
			Self::Column => {
				sink.write_whitespace()?;
				sink.write_char('|')?;
				sink.write_char('|')?;
				sink.write_whitespace()
			}
		}
	}
}
