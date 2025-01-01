use crate::{AtRule, Parse, Parser, Result, T};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a block which can only accept "At Rules".
/// This is an implementation of [`<at-rule-list>`][1].
///
/// This includes the `{` and `}` tokens as this allows for easier expression of blocks within AST nodes. Additionally,
/// it factors in error tolerance where the closing curly can be omitted if the parser reaches the end of the source.
///
/// Every item in the list must implement the [AtRule][crate::AtRule] trait.
///
/// The effective grammar for this trait is:
///
/// ```md
/// <at-rule-list>
///  │├─ "{" ─╭─ <at-rule> ─╮─╮─ "}" ─╭──┤│
///           ╰─────────────╯ ╰───────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
pub trait AtRuleList<'a>: Sized + Parse<'a> {
	type AtRule: AtRule<'a>;

	fn parse_at_rule_list(p: &mut Parser<'a>) -> Result<(T!['{'], Vec<'a, Self::AtRule>, Option<T!['}']>)> {
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
			rules.push(p.parse::<Self::AtRule>()?);
		}
	}
}
