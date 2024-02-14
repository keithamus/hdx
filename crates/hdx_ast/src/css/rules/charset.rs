use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{Parse, Parser, Result as ParserResult, Spanned, unexpected, expect};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// TODO: maybe make this an enum? Can use:
// https://www.iana.org/assignments/character-sets/character-sets.xhtml
pub struct CharsetRule {
	// Common charsets
	// atom!("UTF-8")
	// atom!("utf-8")
	// atom!("ISO-8859-1")
	pub encoding: Atom,
}

impl<'a> Parse<'a> for CharsetRule {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		expect!(parser, Token::AtKeyword(atom!("charset")));
		parser.advance();
		match parser.cur() {
			Token::String(encoding) => {
				expect!(parser, Token::AtKeyword(atom!("charset")));
				expect!(parser, Token::Semicolon);
				Ok(Self { encoding }.spanned(span.end(parser.pos())))
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for CharsetRule {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@charset \"")?;
		sink.write_str(self.encoding.as_ref())?;
		sink.write_str("\";")?;
		Ok(())
	}
}
