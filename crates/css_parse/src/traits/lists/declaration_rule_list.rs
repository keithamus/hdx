use crate::{diagnostics, AtRule, Declaration, Parse, Parser, Result, T};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a rule's block that is only capable of having child declarations
/// or at-rules. Qualified Rules are not allowed. It is an [implementation of "declaration-rule-list"][1]. It includes
/// an error tolerance in that the ending `}` token can be omitted, if at the end of the file. Additionally the `;`
/// token may or may not be present on declarations.
///
/// [DeclarationRuleList::Declaration] refers to the `<declataion>` grammar and is required to implement the
/// [Declaration][crate::Declaration] trait. [DeclarationRuleList::AtRule] refers to the <at-rule> grammar and is
/// required to impement the [AtRule][crate::AtRule] trait.
///
/// ```md
/// <declaration-rule-list>
///  │├─ "{" ─╮─╭─╮─ <declaration> ─╮─ ";" ─╭─╮─╭─╮─ "}" ─╭─┤│
///           │ │ │                 ╰───────╯ │ │ ╰───────╯
///           │ │ ╰─ <at-rule> ───────────────┤ │
///           │ ╰─────────────────────────────╯ │
///           ╰─────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
pub trait DeclarationRuleList<'a>: Sized + Parse<'a> {
	type Declaration: Declaration<'a>;
	type AtRule: AtRule<'a>;

	fn parse_declaration_rule_list(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, (Self::Declaration, Option<T![;]>)>, Vec<'a, Self::AtRule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok((left, declarations, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, declarations, rules, Some(p.parse::<T!['}']>()?)));
			}
			if p.peek::<T![AtKeyword]>() {
				rules.push(p.parse::<Self::AtRule>()?);
			} else if p.peek::<T![Ident]>() {
				let rule = p.parse::<Self::Declaration>()?;
				let semi = p.parse_if_peek::<T![;]>()?;
				declarations.push((rule, semi));
			} else {
				let c = p.peek_n(1);
				Err(diagnostics::Unexpected(c.into(), c.into()))?;
			}
		}
	}
}
