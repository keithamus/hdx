use crate::{Parse, Parser, Result, State, T};
use bumpalo::collections::Vec;

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, Self::Declaration>, Vec<'a, Self::Rule>, Option<T!['}']>)> {
		let open = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				break;
			}
			p.parse_if_peek::<T![;]>().ok();
			if p.peek::<T!['}']>() {
				break;
			}
			let old_state = p.set_state(State::Nested);
			if p.peek::<T![AtKeyword]>() {
				rules.push(p.parse::<Self::Rule>().inspect_err(|_| {
					p.set_state(old_state);
				})?);
			} else {
				let checkpoint = p.checkpoint();

				if let Ok(decl) = p.parse::<Self::Declaration>() {
					declarations.push(decl);
				} else {
					p.rewind(checkpoint);
					rules.push(p.parse::<Self::Rule>()?);
				}
			}
			p.set_state(old_state);
		}
		Ok((open, declarations, rules, p.parse_if_peek::<T!['}']>()?))
	}
}
