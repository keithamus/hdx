use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::Kind;
use hdx_parser::{Parse, Parser, Result as ParserResult};

use crate::css::types::Color;

// https://drafts.csswg.org/css-text-decor/#text-decoration-color-property
#[derive(Value, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TextDecorationColor {
	#[default]
	Auto, // atom!("auto")
	Color(Color),
}

impl<'a> Parse<'a> for TextDecorationColor {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.cur().kind() {
			Kind::Ident if parser.parse_atom_lower(parser.cur()) == atom!("auto") => Self::Auto,
			_ => Self::Color(Color::parse(parser)?),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextDecorationColor, 36);
	}
}
