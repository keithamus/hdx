use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::Kind;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use crate::css::types::Image;

// https://drafts.csswg.org/css-lists/#list-style-property
#[derive(Value, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ListStyleImage {
	Image(Image),
	#[default]
	None,
}

impl<'a> Parse<'a> for ListStyleImage {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.peek();
		Ok(match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("none") => {
					parser.next();
					Self::None
				}
				atom => unexpected_ident!(parser, atom),
			},
			Kind::Function | Kind::Url => Self::Image(Image::parse(parser)?),
			_ => unexpected!(parser, token),
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
