use crate::{diagnostics, Parse, Parser, Result, T};
use hdx_lexer::Include;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Comparison {
	LessThan,         // '<'
	GreaterThan,      // '>'
	GreaterThanEqual, // '>='
	LessThanEqual,    // '<='
	Equal,            // '='
}

impl<'a> Parse<'a> for Comparison {
	fn parse(parser: &mut Parser<'a>) -> Result<Comparison> {
		let token = *parser.parse::<T![Delim]>()?;
		match token.char().unwrap() {
			'=' => Ok(Comparison::Equal),
			'>' => {
				if let Some(token) = parser.peek_with::<T![Delim]>(Include::Whitespace) {
					if let Some('=') = token.char() {
						parser.hop(token);
						return Ok(Comparison::GreaterThanEqual);
					}
				}
				Ok(Comparison::GreaterThan)
			}
			'<' => {
				if let Some(token) = parser.peek_with::<T![Delim]>(Include::Whitespace) {
					if let Some('=') = token.char() {
						parser.hop(token);
						return Ok(Comparison::LessThanEqual);
					}
				}
				Ok(Comparison::LessThan)
			}
			char => Err(diagnostics::UnexpectedDelim(char, token.span()))?,
		}
	}
}
