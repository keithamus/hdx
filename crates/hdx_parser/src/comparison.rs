use crate::match_delim;
use crate::{unexpected, Parse, Parser, Result};
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
		let next = parser.next();
		Ok(match_delim! {parser.next() :
		  '=' => Comparison::Equal,
		  '>' => {
				match_delim!{ parser.peek_with(Include::Whitespace) :
					'=' => {
					   parser.advance_with(Include::Whitespace);
					   Comparison::GreaterThanEqual
					},
					_ =>  Comparison::GreaterThan
				}
			},
			'<' => {
				match_delim!{ parser.peek_with(Include::Whitespace) :
					'=' => {
								parser.advance_with(Include::Whitespace);
								Comparison::LessThanEqual
					},
					_ => Comparison::LessThan
				}
			},
			_ =>  unexpected!(parser, next)
		})
	}
}
