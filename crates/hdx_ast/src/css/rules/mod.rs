pub mod charset;
pub mod page;

pub use charset::*;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Result as ParserResult, Spanned, Parser};
pub use page::*;

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::LeftCurly | Token::Semicolon => Ok(Self {}.spanned(span.end(parser.pos()))),
			token => unexpected!(parser, token)
		}
	}
}

pub struct NoBlockAllowed;
impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Semicolon | Token::Eof => Ok(Self {}.spanned(span.end(parser.pos()))),
			token => unexpected!(parser, token)
		}
	}
}
