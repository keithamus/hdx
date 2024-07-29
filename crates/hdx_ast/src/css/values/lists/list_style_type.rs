use hdx_atom::{atom, Atom};
use hdx_derive::{Value, Writable};
use hdx_lexer::{QuoteStyle, Kind};
use hdx_parser::{Parse, Parser, Result as ParserResult, Spanned};

use crate::css::types::CounterStyle;

// https://drafts.csswg.org/css-lists/#list-style-property
#[derive(Writable, Value, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ListStyleType {
	CounterStyle(Spanned<CounterStyle>),
	#[writable(String)]
	String(Atom, QuoteStyle),
	None,
}

impl<'a> Parse<'a> for ListStyleType {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.peek();
		Ok(match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("none") => {
					parser.next();
					Self::None
				}
				_ => Self::CounterStyle(CounterStyle::parse_spanned(parser)?),
			},
			Kind::String => {
				parser.next();
				Self::String(parser.parse_atom(token), token.quote_style())
			}
			_ => Self::CounterStyle(CounterStyle::parse_spanned(parser)?),
		})
	}
}

impl Default for ListStyleType {
	fn default() -> Self {
		Self::CounterStyle(Spanned::dummy(CounterStyle::default()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ListStyleType, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ListStyleType, "decimal");
		assert_parse!(ListStyleType, "lower-alpha");
		assert_parse!(ListStyleType, "none");
		assert_parse!(ListStyleType, "custom-counter-style");
		assert_parse!(ListStyleType, "\"-\"");
		assert_parse!(ListStyleType, "symbols(symbolic '-')");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(ListStyleType, "");
	}
}
