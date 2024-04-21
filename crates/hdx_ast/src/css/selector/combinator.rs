use hdx_lexer::{Include, Token};
use hdx_parser::{discard, expect, peek, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	Column,            // ||
}

impl<'a> Parse<'a> for Combinator {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let could_be_descendant_combinator = discard!(parser, Include::Whitespace, Token::Whitespace);
		if !peek!(parser, Token::Delim(_)) && could_be_descendant_combinator {
			return Ok(Self::Descendant);
		}
		let val = match parser.next() {
			Token::Delim(c) => match c {
				'>' => Self::Child,
				'+' => Self::NextSibling,
				'~' => Self::SubsequentSibling,
				'|' => {
					expect!(parser.next_with(Include::Whitespace), Token::Delim('|'));
					Self::Column
				}
				_ => unexpected!(parser),
			},
			token => unexpected!(parser, token),
		};
		discard!(parser, Include::Whitespace, Token::Whitespace);
		Ok(val)
	}
}

impl<'a> WriteCss<'a> for Combinator {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Descendant => sink.write_char(' '),
			Self::Child => {
				sink.write_whitespace()?;
				sink.write_char('>')?;
				sink.write_whitespace()
			}
			Self::NextSibling => {
				sink.write_whitespace()?;
				sink.write_char('+')?;
				sink.write_whitespace()
			}
			Self::SubsequentSibling => {
				sink.write_whitespace()?;
				sink.write_char('~')?;
				sink.write_whitespace()
			}
			Self::Column => {
				sink.write_whitespace()?;
				sink.write_char('|')?;
				sink.write_char('|')?;
				sink.write_whitespace()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Combinator, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Combinator, ">", " > ");
		assert_parse!(Combinator, "+", " + ");
		assert_parse!(Combinator, "~", " ~ ");
		// Descendent combinator
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "  /**/   /**/   /**/ ", " ");
		// Column
		assert_parse!(Combinator, "||", " || ");
		assert_parse!(Combinator, " || ", " || ");
	}
}
