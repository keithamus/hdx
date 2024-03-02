pub mod charset;
pub mod page;
pub mod media;
pub mod supports;

pub use charset::*;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
pub use page::*;
pub use media::*;
pub use supports::*;

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::LeftCurly | Token::Semicolon => Ok(Self {}),
			token => unexpected!(parser, token),
		}
	}
}

pub struct NoBlockAllowed;
impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Semicolon | Token::Eof => Ok(Self {}),
			token => unexpected!(parser, token),
		}
	}
}
