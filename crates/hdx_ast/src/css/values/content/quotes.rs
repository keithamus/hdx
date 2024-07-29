use hdx_atom::{atom, Atom};
use hdx_derive::Value;
use hdx_lexer::{QuoteStyle, Kind};
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

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
		let token = parser.next();
		Ok(match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("none") => Quotes::None,
				atom!("auto") => Quotes::Auto,
				atom!("match-parent") => Quotes::MatchParent,
				atom => unexpected_ident!(parser, atom),
			},
			Kind::String => {
				let mut quotes = smallvec![(parser.parse_atom_lower(token), token.quote_style())];
				loop {
					let token = parser.peek();
					if token.kind() != Kind::String {
						break;
					}
					parser.next();
					quotes.push((parser.parse_atom_lower(token), token.quote_style()));
				}
				if quotes.len() % 2 != 0 {
					unexpected!(parser, parser.peek());
				}
				Quotes::Pairs(quotes)
			}
			_ => unexpected!(parser, token),
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
