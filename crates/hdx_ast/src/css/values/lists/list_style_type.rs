use hdx_atom::{atom, Atom};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{Parse, Parser, Result as ParserResult, Spanned};

use crate::{css::types::CounterStyle, Value, Writable};

// https://drafts.csswg.org/css-lists/#list-style-property
#[derive(Writable, Value, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ListStyleType {
	CounterStyle(Spanned<CounterStyle>),
	#[writable(String)]
	String(Atom, QuoteStyle),
	None,
}

impl<'a> Parse<'a> for ListStyleType {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek().clone() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Self::None
				}
				_ => Self::CounterStyle(CounterStyle::parse_spanned(parser)?),
			},
			Token::String(atom, style) => {
				parser.advance();
				Self::String(atom, style)
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
