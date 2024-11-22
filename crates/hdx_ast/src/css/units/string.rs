use hdx_derive::Writable;
use hdx_lexer::QuoteStyle;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
use std::fmt::{Display, Result as DisplayResult};

// Some CSS values include quoted strings. They use this unit to represent that.
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CSSString<'a>(&'a str, QuoteStyle);

impl<'a> CSSString<'a> {
	pub fn to_string(&self) -> String {
		self.0.to_string()
	}
}

impl<'a> Display for CSSString<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DisplayResult {
		self.0.fmt(f)
	}
}

impl<'a> Peek<'a> for CSSString<'a> {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<T![String]>().filter(|t| !t.is_float())
	}
}

impl<'a> Parse<'a> for CSSString<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<T![String]>()?;
		let str = parser.parse_str(token);
		Ok(Self(str, token.quote_style()))
	}
}
