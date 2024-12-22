use crate::{Declaration, Parse, Parser, Result, T};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a rule's block, that is only capable of having child declarations.
/// It is an [implementation of "declaration-list"][1]. It includes an error tolerance in that the ending `}` token can
/// be omitted, if at the end of the file. Additionally the `;` token may or may not be present.
///
/// [DeclarationList::Declaration] refers to the `<declataion>` grammar and is required to implement the
/// [Declaration][crate::Declaration] trait.
///
/// ```md
/// <declaration-list>
///  │├─ "{" ─╮─╭─ <declaration> ─╮─ ";" ─╭──╮─╭─╮─ "}" ─╭─┤│
///           │ │                 ╰───────╯  │ │ ╰───────╯
///           │ ╰────────────────────────────╯ │
///           ╰────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
pub trait DeclarationList<'a>: Sized + Parse<'a> {
	type Declaration: Declaration<'a>;

	fn parse_declaration_list(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, (Self::Declaration, Option<T![;]>)>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			let rule = p.parse::<Self::Declaration>()?;
			let semi = p.parse_if_peek::<T![;]>()?;
			rules.push((rule, semi));
		}
	}
}
