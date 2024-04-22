use hdx_lexer::{Token, Include};
use crate::{State, Parse, Parser, Result, unexpected};

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
		Ok(match parser.next() {
			Token::Delim('=') => Comparison::Equal,
			Token::Delim('>') => {
				if let Token::Delim('=') = parser.peek_with(Include::Whitespace) {
					parser.advance_with(Include::Whitespace);
					Comparison::GreaterThanEqual
				} else {
					Comparison::GreaterThan
				}
			}
			Token::Delim('<') => {
				if let Token::Delim('=') = parser.peek_with(Include::Whitespace) {
					parser.advance_with(Include::Whitespace);
					Comparison::LessThanEqual
				} else {
					Comparison::LessThan
				}
			}
			token => unexpected!(parser, token),
		})
	}
}
