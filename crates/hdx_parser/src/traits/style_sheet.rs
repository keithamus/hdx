use crate::{Parse, Parser, Result, T};
use bumpalo::collections::Vec;

pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(p: &mut Parser<'a>) -> Result<Vec<'a, Self::Rule>> {
		let mut rules: Vec<'a, Self::Rule> = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok(rules);
			}
			p.parse_if_peek::<T![CdcOrCdo]>()?;
			if let Ok(rule) = p.parse::<Self::Rule>() {
				rules.push(rule)
			}
		}
	}
}
