use crate::match_token_kind_and_char;
use crate::{unexpected, Parse, Parser, Result};
use hdx_lexer::{Include, Kind};

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
		let next = parser.next();
		Ok(if match_token_kind_and_char!(next, Kind::Delim, '=') {
			Comparison::Equal
		} else if match_token_kind_and_char!(next, Kind::Delim, '>') {
			let peek = parser.peek_with(Include::Whitespace);
			if match_token_kind_and_char!(peek, Kind::Delim, '=') {
				parser.advance_with(Include::Whitespace);
				Comparison::GreaterThanEqual
			} else {
				Comparison::GreaterThan
			}
		} else if match_token_kind_and_char!(next, Kind::Delim, '<') {
			let peek = parser.peek_with(Include::Whitespace);
			if match_token_kind_and_char!(peek, Kind::Delim, '=') {
				parser.advance_with(Include::Whitespace);
				Comparison::LessThanEqual
			} else {
				Comparison::LessThan
			}
		} else {
			unexpected!(parser, next)
		})
	}
}
