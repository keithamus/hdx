use hdx_atom::atom;
use hdx_lexer::QuoteStyle;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use super::Gradient;

mod func {
	use hdx_parser::custom_function;
	custom_function!(Url, atom!("url"));
}

// https://drafts.csswg.org/css-images-3/#typedef-image
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Image<'a> {
	Url(&'a str, QuoteStyle),
	Gradient(Gradient),
}

impl<'a> Peek<'a> for Image<'a> {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Url]>().or_else(|| p.peek::<func::Url>()).or_else(|| p.peek::<Gradient>())
	}
}

impl<'a> Parse<'a> for Image<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = p.peek::<T![Url]>() {
			p.hop(token);
			return Ok(Self::Url(p.parse_str(token), token.quote_style()));
		}
		if let Some(token) = p.peek::<func::Url>() {
			p.hop(token);
			let string_token = p.parse::<T![String]>()?;
			p.parse::<T![RightParen]>()?;
			return Ok(Self::Url(p.parse_str(*string_token), string_token.quote_style()));
		}
		p.parse::<Gradient>().map(Self::Gradient)
	}
}

impl<'a> WriteCss<'a> for Image<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Url(str, style) => {
				atom!("url").write_css(sink)?;
				sink.write_char('(')?;
				sink.write_with_quotes(str, *style, true)?;
				sink.write_char(')')
			}
			Self::Gradient(g) => g.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Image, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image, "url('foo')");
		assert_parse!(Image, "url(\"foo\")");
		assert_parse!(Image, "url(foo)");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Image, "url('foo')", "url(foo)");
	}
}
