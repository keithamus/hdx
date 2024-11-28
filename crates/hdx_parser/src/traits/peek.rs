use crate::{Is, Parser};

pub trait Peek<'a>: Sized {
	fn peek(p: &Parser<'a>) -> bool;
}

impl<'a, T: Is<'a>> Peek<'a> for T {
	fn peek(p: &Parser<'a>) -> bool {
		T::is(p, p.peek_next().into())
	}
}
