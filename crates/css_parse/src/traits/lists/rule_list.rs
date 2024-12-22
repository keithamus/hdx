use crate::{Parse, Parser, Result, T};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a rule's block that is only capable of having child qualified
/// rules or at-rules. Declarations are not allowed. It is an [implementation of "rule-list"][1]. It includes an error
/// tolerance in that the ending `}` token can be omitted, if at the end of the file.
///
/// [RuleList::Rule] refers to either the `<qualified-rule>` or the `<at-rule>`. Rather than having these as separate
/// nodes which would require an intermediary struct like `Either`, they are exposed as one node type which can be more
/// ergonomically expressed in the AST.
///
/// ```md
/// <qualified-rule>
/// │├─ <rule> ─┤│
///
/// <at-rule>
/// │├─ <rule> ─┤│
///
/// <declaration-rule-list>
///  │├─ "{" ─╮─╭─╮─ <qualified-rule> ────────╮─╭─╮─ "}" ─╭─┤│
///           │ │ ╰─ <at-rule> ───────────────┤ │ ╰───────╯
///           │ ╰─────────────────────────────╯ │
///           ╰─────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
// <rule-list>: qualified rules and at-rules are allowed; declarations are automatically invalid.
pub trait RuleList<'a>: Sized + Parse<'a> {
	// To simplify typings and orderings, a generic "Rule" type houses both At/Qualified rules
	type Rule: Parse<'a>;

	fn parse_rule_list(p: &mut Parser<'a>) -> Result<(T!['{'], Vec<'a, Self::Rule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			rules.push(p.parse::<Self::Rule>()?);
		}
	}
}
