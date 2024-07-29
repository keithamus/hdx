use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::Token;
use hdx_parser::{Parse, Parser, Result as ParserResult};

use crate::css::types::Color;

// https://drafts.csswg.org/css-ui/#widget-accent
// auto | <color> 
#[derive(Value, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AccentColor {
	#[default]
	Auto, // atom!("auto")
	Color(Color),
}

impl<'a> Parse<'a> for AccentColor {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(if peek_ignore_case!(parser, Kind::Ident, atom!("auto")) {
			Self::Auto
		} else {
			Self::Color(Color::parse(parser)?)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AccentColor, 36);
	}
}
