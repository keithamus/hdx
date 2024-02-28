use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use hdx_writer::WriteCss;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Parsable, Value, Vec, Writable};

#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Content {
	#[default]
	Normal,
	None,
	// Image(),  // TODO: Implement image
	String(Atom),
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
			Token::String(atom) => {
				parser.advance();
				Self::String(atom)
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
			Self::String(str) => {
				sink.write_char('"')?;
				sink.write_str(str)?;
				sink.write_char('"')
			}
		}
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
		assert_eq!(size_of::<Content>(), 16);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Content>(&allocator, "none", "none");
		test_write::<Content>(&allocator, "'foo'", "\"foo\"");
	}
}
