use hdx_lexer::Token;
use hdx_parser::{expect, unexpected, Parse, Parser, Result as ParserResult};
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
		match parser.cur() {
			Token::Whitespace => {
				loop {
					parser.advance_including_whitespace();
					if !matches!(parser.cur(), Token::Whitespace) {
						break;
					}
				}
				Ok(Self::Descendant)
			}
			Token::Delim(c) => match c {
				'>' => Ok(Self::Child),
				'+' => Ok(Self::NextSibling),
				'~' => Ok(Self::SubsequentSibling),
				'|' => {
					parser.advance_including_whitespace();
					expect!(parser, Token::Delim('|'));
					parser.advance_including_whitespace();
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
			Self::Child => sink.write_char('~'),
			Self::NextSibling => sink.write_char('+'),
			Self::SubsequentSibling => sink.write_char('~'),
			Self::Column => {
				sink.write_char('|')?;
				sink.write_char('|')
			}
		}
	}
}
