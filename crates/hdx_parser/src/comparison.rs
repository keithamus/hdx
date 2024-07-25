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
		Ok(match (parser.next().kind(), parser.next().char()) {
			(Kind::Delim, Some('=')) => Comparison::Equal,
			(Kind::Delim, Some('>')) => {
				if let (Kind::Delim, Some('=')) =
					(parser.peek_with(Include::Whitespace).kind(), parser.peek_with(Include::Whitespace).char())
				{
					parser.advance_with(Include::Whitespace);
					Comparison::GreaterThanEqual
				} else {
					Comparison::GreaterThan
				}
			}
			(Kind::Delim, Some('<')) => {
				if let (Kind::Delim, Some('=')) =
					(parser.peek_with(Include::Whitespace).kind(), parser.peek_with(Include::Whitespace).char())
				{
					parser.advance_with(Include::Whitespace);
					Comparison::LessThanEqual
				} else {
					Comparison::LessThan
				}
			}
			(token, _) => unexpected!(parser, token),
		})
	}
}
