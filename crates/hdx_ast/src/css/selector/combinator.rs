use hdx_lexer::{Include, Token};
use hdx_parser::{discard, expect, peek, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss, write_css};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	Column,            // ||
	Nesting,           // &
}

impl<'a> Parse<'a> for Combinator {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let could_be_descendant_combinator = discard!(parser, Include::Whitespace, Token::Whitespace);
		if !peek!(parser, Token::Delim(_)) && could_be_descendant_combinator {
			return Ok(Self::Descendant);
		}
		let val = match parser.peek() {
			Token::Delim(c) => match c {
				'>' => Self::Child,
				'+' => Self::NextSibling,
				'~' => Self::SubsequentSibling,
				'&' => Self::Nesting,
				'|' => {
					expect!(parser.next_with(Include::Whitespace), Token::Delim('|'));
					Self::Column
				}
				_ if could_be_descendant_combinator => return Ok(Self::Descendant),
				_ => unexpected!(parser),
			},
			token => unexpected!(parser, token),
		};
		parser.advance();
		discard!(parser, Include::Whitespace, Token::Whitespace);
		Ok(val)
	}
}

impl<'a> WriteCss<'a> for Combinator {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Descendant => sink.write_char(' ')?,
			Self::Nesting => write_css!(sink, (), '&', ()),
			Self::Child => write_css!(sink, (), '>', ()),
			Self::NextSibling => write_css!(sink, (), '+', ()),
			Self::SubsequentSibling => write_css!(sink, (), '~', ()),
			Self::Column => write_css!(sink, (), '|', '|', ()),
		}
		Ok(())
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
