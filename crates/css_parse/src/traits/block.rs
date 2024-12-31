use crate::{Parse, Parser, Result, State, T};
use bumpalo::collections::Vec;

use super::Peek;

/// This trait provides an implementation for ["consuming a blocks contents"][1].
///
/// ```md
/// <block>
///          
///  │├─ "{" ─╭──╮─╭─ <ws-*> ─╮╭─ ";" ─╮─╭─╮─ <rule> ────────╭─╮─ "}" ─┤│
///           │  │ ╰──────────╯╰───────╯ │ ├─ <declaration> ─┤ │
///           │  ╰───────────────────────╯ ╰─────────────────╯ │
///           ╰────────────────────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-block-contents
pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Peek<'a> + Parse<'a>;
	type Rule: Parse<'a>;

	fn parse_block(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, (Self::Declaration, Option<T![;]>)>, Vec<'a, Self::Rule>, Option<T!['}']>)> {
		let open = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		loop {
			// While by default the parser will skip whitespace, the Declaration or Rule type may be a whitespace sensitive
			// node, for example `ComponentValues`. As such whitespace needs to be consumed here, before Declarations and
			// Rules are parsed.
			if p.parse_if_peek::<T![' ']>()?.is_some() || p.parse_if_peek::<T![;]>()?.is_some() {
				continue;
			}
			if p.at_end() {
				break;
			}
			if p.peek::<T!['}']>() {
				break;
			}
			let old_state = p.set_state(State::Nested);
			if p.peek::<T![AtKeyword]>() {
				let rule = p.parse::<Self::Rule>();
				p.set_state(old_state);
				rules.push(rule?);
			} else if let Ok(Some(decl)) = p.try_parse_if_peek::<Self::Declaration>() {
				p.set_state(old_state);
				declarations.push((decl, p.parse_if_peek::<T![;]>()?));
			} else {
				let rule = p.parse::<Self::Rule>();
				p.set_state(old_state);
				rules.push(rule?);
			}
		}
		Ok((open, declarations, rules, p.parse_if_peek::<T!['}']>()?))
	}
}
