use hdx_atom::{atom, Atom};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use hdx_writer::WriteCss;

use crate::Value;

#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Content {
	#[default]
	Normal,
	None,
	// Image(),  // TODO: Implement image
	String(Atom, QuoteStyle),
	// CounterFunction(), // TODO: Implement counter()
	// CountersFunction(), // TODO: Implement counters()
	// ContentFunction(), // TODO: Implement content()
	// AttrFunction(), // TODO: Implement attr()
}

impl<'a> Parse<'a> for Content {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let value = match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("normal") => {
					parser.advance();
					Self::Normal
				}
				atom!("none") => {
					parser.advance();
					Self::None
				}
				atom => unexpected_ident!(parser, atom),
			},
			Token::String(atom, quote) => {
				parser.advance();
				Self::String(atom, quote)
			}
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for Content {
	fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
		match self {
			Self::None => atom!("none").write_css(sink),
			Self::Normal => atom!("normal").write_css(sink),
			Self::String(str, quote) => sink.write_with_quotes(str.as_ref(), *quote, false),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Content, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Content, "none");
		assert_parse!(Content, "'foo'");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Content, "none", "none");
		assert_minify!(Content, "'foo'", "\"foo\"");
	}
}
