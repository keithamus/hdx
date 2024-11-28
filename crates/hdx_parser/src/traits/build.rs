use hdx_lexer::Cursor;

use crate::Parser;

pub trait Build<'a>: Sized {
	fn build(p: &Parser<'a>, c: Cursor) -> Self;
}
