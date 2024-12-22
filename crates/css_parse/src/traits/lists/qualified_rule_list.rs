use crate::{Parse, Parser, QualifiedRule, Result, T};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a rule's block that is only capable of having child qualified
/// rules. At Rules and Declarations are not allowed. It is an [implementation of "declaration-list"][1]. It includes
/// an error tolerance in that the ending `}` token can be omitted, if at the end of the file.
///
/// [QualifiedRuleList::QualifiedRule] refers to the `<declataion>` grammar and is required to implement the
/// [QualifiedRule][crate::QualifiedRule] trait.
///
/// ```md
/// <qualified-rule-list>
///  │├─ "{" ─╮─╭─ <qualified-rule> ─╮─ ";" ─╭──╮─╭─╮─ "}" ─╭─┤│
///           │ │                    ╰───────╯  │ │ ╰───────╯
///           │ ╰───────────────────────────────╯ │
///           ╰───────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-qualified-rule-list
pub trait QualifiedRuleList<'a>: Sized + Parse<'a> {
	type QualifiedRule: QualifiedRule<'a>;

	fn parse_qualified_rule_list(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, Self::QualifiedRule>, Option<T!['}']>)> {
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
			rules.push(p.parse::<Self::QualifiedRule>()?);
		}
	}
}
