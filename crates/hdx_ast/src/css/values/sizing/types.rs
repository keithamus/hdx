use hdx_parser::{CursorStream, Parse, Parser, Peek, Result as ParserResult, ToCursors};

pub use crate::css::units::*;

mod func {
	use hdx_parser::custom_function;
	custom_function!(CalcSize, atom!("calc-size"));
	custom_function!(FitContent, atom!("fit-content"));
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CalcSize;

impl<'a> Peek<'a> for CalcSize {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::CalcSize>()
	}
}

impl<'a> Parse<'a> for CalcSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<func::CalcSize>()?;
		todo!();
	}
}

impl<'a> ToCursors<'a> for CalcSize {
	fn to_cursors(&self, _: &mut CursorStream<'a>) {
		todo!();
	}
}
