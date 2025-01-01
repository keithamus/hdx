use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub use crate::units::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CalcSize;

impl<'a> Peek<'a> for CalcSize {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "calc-size")
	}
}

impl<'a> Parse<'a> for CalcSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<T![Function]>()?;
		todo!();
	}
}

impl<'a> ToCursors for CalcSize {
	fn to_cursors(&self, _: &mut impl CursorSink) {
		todo!();
	}
}
