use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use crate::{css::types::Image, Value, Writable};

// https://drafts.csswg.org/css-lists/#list-style-property
#[derive(Writable, Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ListStyleImage {
	Image(Image),
	#[default]
	None,
}

impl<'a> Parse<'a> for ListStyleImage {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Self::None
				}
				_ => unexpected_ident!(parser, atom),
			},
			Token::Function(_) | Token::Url(_, _) => Self::Image(Image::parse(parser)?),
			token => unexpected!(parser, token),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ListStyleImage, 72);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ListStyleImage, "none");
		assert_parse!(ListStyleImage, "url(foo)");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(ListStyleImage, "");
	}
}
