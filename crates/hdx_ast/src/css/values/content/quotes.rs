use hdx_atom::{atom, Atom};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::Value;

#[derive(Value, Default, Debug, Clone, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Quotes {
	None,
	#[default]
	Auto,
	MatchParent,
	Pairs(SmallVec<[(Atom, QuoteStyle); 2]>),
}

impl<'a> Parse<'a> for Quotes {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Quotes::None
				}
				atom!("auto") => {
					parser.advance();
					Quotes::Auto
				}
				atom!("match-parent") => {
					parser.advance();
					Quotes::MatchParent
				}
				_ => unexpected_ident!(parser, atom),
			},
			Token::String(_, _) => {
				let mut quotes = smallvec![];
				loop {
					let first = if let Token::String(atom, style) = parser.cur() {
						parser.advance();
						(atom, style)
					} else {
						break;
					};
					let second = if let Token::String(atom, style) = parser.cur() {
						parser.advance();
						(atom, style)
					} else {
						unexpected!(parser)
					};
					quotes.push(first);
					quotes.push(second);
				}
				Quotes::Pairs(quotes)
			}
			token => unexpected!(parser, token),
		})
	}
}

impl<'a> WriteCss<'a> for Quotes {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => atom!("none").write_css(sink),
			Self::Auto => atom!("auto").write_css(sink),
			Self::MatchParent => atom!("match-parent").write_css(sink),
			Self::Pairs(pairs) => {
				let mut iter = pairs.iter().peekable();
				while let Some((str, quote)) = iter.next() {
					sink.write_with_quotes(str, *quote, false)?;
					if iter.peek().is_some() {
						sink.write_whitespace()?;
					}
				}
				Ok(())
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
		assert_size!(Quotes, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Quotes, "none");
		assert_parse!(Quotes, "\"“\" \"”\" \"‘\" \"’\"");
	}

	#[test]
	fn test_minify() {
		// Whitespace is redundant
		assert_minify!(Quotes, "\"“\" \"”\" \"‘\" \"’\"", "\"“\"\"”\"\"‘\"\"’\"");
		assert_minify!(Quotes, "\"\\201C\" \"\\201D\" \"\\2018\" \"\\2019\"", "\"“\"\"”\"\"‘\"\"’\"");
	}
}
