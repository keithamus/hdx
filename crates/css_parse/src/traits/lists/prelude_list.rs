use crate::{Parse, Parser, Result};
use bumpalo::collections::Vec;
use css_lexer::KindSet;

pub trait PreludeList<'a>: Sized + Parse<'a> {
	type PreludeItem: Parse<'a>;
	const STOP_TOKENS: KindSet = KindSet::LEFT_CURLY_OR_SEMICOLON;

	fn parse_prelude_list(p: &mut Parser<'a>) -> Result<Vec<'a, Self::PreludeItem>> {
		let mut items = Vec::new_in(p.bump());
		loop {
			items.push(p.parse::<Self::PreludeItem>()?);
			if p.peek_next() == Self::STOP_TOKENS {
				return Ok(items);
			}
		}
	}
}
