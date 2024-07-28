use crate::{unexpected, expect, Parse, Parser, Result};
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
		Ok(match expect!(parser.next(), Kind::Delim).char().unwrap() {
			'=' => Comparison::Equal,
			'>' if matches!(parser.peek_with(Include::Whitespace).char(), Some('=')) => {
				parser.next_with(Include::Whitespace);
				Comparison::GreaterThanEqual
			},
			'>' =>  Comparison::GreaterThan,
			'<' if matches!(parser.peek_with(Include::Whitespace).char(), Some('=')) => {
				parser.next_with(Include::Whitespace);
				Comparison::LessThanEqual
			},
			'<' => Comparison::LessThan,
			_ => unexpected!(parser),
		})
	}
}
