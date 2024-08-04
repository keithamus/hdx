use hdx_atom::atom;
use hdx_derive::{from_syntax, Value};
use hdx_lexer::{Kind, Token};
use hdx_parser::{Parse, Parser, Result as ParserResult};

// https://drafts.csswg.org/css-ui/#caret-color
#[from_syntax(auto | <color>)]
#[derive(Value)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum CaretColor {}

// impl<'a> Parse<'a> for CaretColor {
// 	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
// 		Ok(match parser.cur().kind() {
// 			Kind::Ident if parser.parse_atom_lower(parser.cur()) == atom!("auto") => Self::Auto,
// 			_ => Self::Color(Color::parse(parser)?),
// 		})
// 	}
// }
//
#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CaretColor, 36);
	}
}
