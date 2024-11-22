use hdx_atom::atom;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use super::CSSFloat;

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Flex(CSSFloat);

impl From<f32> for Flex {
	fn from(value: f32) -> Self {
		Self(value.into())
	}
}

impl From<&f32> for Flex {
	fn from(value: &f32) -> Self {
		Self(value.into())
	}
}

impl From<CSSFloat> for Flex {
	fn from(value: CSSFloat) -> Self {
		Self(value)
	}
}

impl<'a> Peek<'a> for Flex {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<T![Dimension::Fr]>()
	}
}

impl<'a> Parse<'a> for Flex {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<T![Dimension::Fr]>()?;
		Ok(parser.parse_number(token).into())
	}
}

impl<'a> WriteCss<'a> for Flex {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_css!(sink, self.0, atom!("fr"));
		Ok(())
	}
}
