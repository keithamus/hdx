use hdx_atom::{atom, Atom};
use hdx_derive::Value;
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::WriteCss;

#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
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
		Ok(match parser.next() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("normal") => Self::Normal,
				atom!("none") => Self::None,
				atom => unexpected_ident!(parser, atom),
			},
			Token::String(atom, quote) => Self::String(atom.clone(), *quote),
			token => unexpected!(parser, token),
		})
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
