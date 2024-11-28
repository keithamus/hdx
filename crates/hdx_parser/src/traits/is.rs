use hdx_lexer::{Cursor, PairWise};

use crate::Parser;

pub trait Is<'a>: Sized {
	fn is(p: &Parser<'a>, c: Cursor) -> bool;
}

impl<'a> Is<'a> for PairWise {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		c.token().to_pairwise().is_some()
	}
}
