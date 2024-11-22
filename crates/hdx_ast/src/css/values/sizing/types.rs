use hdx_parser::{Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

pub use crate::css::units::*;

mod func {
	use hdx_parser::custom_function;
	custom_function!(CalcSize, atom!("calc-size"));
	custom_function!(FitContent, atom!("fit-content"));
}

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CalcSize;

impl<'a> Peek<'a> for CalcSize {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<func::CalcSize>()
	}
}

impl<'a> Parse<'a> for CalcSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<func::CalcSize>()?;
		todo!();
	}
}

impl<'a> WriteCss<'a> for CalcSize {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		todo!();
	}
}
