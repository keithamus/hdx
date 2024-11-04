use hdx_lexer::Span;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::CSSFloat;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Infinite, atom!("infinite"));
}

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Debug, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SingleAnimationIterationCount {
	Infinite,
	Number(CSSFloat),
}

impl From<f32> for SingleAnimationIterationCount {
	fn from(f: f32) -> Self {
	    Self::Number(f.into())
	}
}

impl<'a> Peek<'a> for SingleAnimationIterationCount {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<kw::Infinite>().or_else(|| parser.peek::<CSSFloat>())
	}
}

impl<'a> Parse<'a> for SingleAnimationIterationCount {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Infinite>() {
			parser.hop(token);
			return Ok(Self::Infinite);
		}
		let start = parser.offset();
		let f = parser.parse::<CSSFloat>()?;
		if f < 0.0 {
			Err(diagnostics::NumberTooSmall(f.into(), Span::new(start, parser.offset())))?
		}
		Ok(Self::Number(f))
	}
}

impl<'a> WriteCss<'a> for SingleAnimationIterationCount {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Infinite => kw::Infinite::atom().write_css(sink),
			Self::Number(f) => f.write_css(sink),
		}
	}
}
