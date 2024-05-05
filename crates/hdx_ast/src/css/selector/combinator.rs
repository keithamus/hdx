use hdx_lexer::{Include, Kind, Token};
use hdx_parser::{discard, expect_delim, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

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
		let peeked = parser.peek();
		if could_be_descendant_combinator && !matches!(peeked.char(), Some('>' | '+' | '~' | '|')) {
			return Ok(Self::Descendant);
		}
		let val = match peeked {
			Token::Delim(c) => match c {
				'>' => Self::Child,
				'+' => Self::NextSibling,
				'~' => Self::SubsequentSibling,
				'&' => Self::Nesting,
				'|' => {
					expect_delim!(parser.next_with(Include::Whitespace), '|');
					Self::Column
				}
				_ if could_be_descendant_combinator => return Ok(Self::Descendant),
				_ => unexpected!(parser),
			},
			token => unexpected!(parser, token),
		};
		parser.advance();
		if val != Self::Nesting {
			discard!(parser, Include::Whitespace, Token::Whitespace);
		}
		Ok(val)
	}
}

impl<'a> WriteCss<'a> for Combinator {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Descendant => sink.write_char(' ')?,
			Self::Nesting => write_css!(sink, '&'),
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
		assert_parse!(Combinator, "&", "&");
		// Descendent combinator
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "  /**/   /**/   /**/ ", " ");
		// Column
		assert_parse!(Combinator, "||", " || ");
		assert_parse!(Combinator, " || ", " || ");
	}
}
