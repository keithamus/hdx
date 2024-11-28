use crate::{diagnostics, Build, Is, Parser, Result};

pub trait Parse<'a>: Sized {
	fn parse(p: &mut Parser<'a>) -> Result<Self>;

	fn try_parse(p: &mut Parser<'a>) -> Result<Self> {
		let checkpoint = p.checkpoint();
		Self::parse(p).inspect_err(|_| p.rewind(checkpoint))
	}
}

impl<'a, T> Parse<'a> for T
where
	T: Sized + Is<'a> + Build<'a>,
{
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self::build(p, c))
		} else {
			let c = p.next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}
