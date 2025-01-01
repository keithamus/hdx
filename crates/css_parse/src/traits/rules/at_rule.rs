use crate::{diagnostics, Parse, Parser, Result, T};
use css_lexer::{Cursor, KindSet};

/// This trait provides an implementation for parsing an [at-rule][1]. The AtRule represents a block or statement with
/// an @keyword in the leading position, such as `@media`, `@charset`, `@import` and so-on.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-at-rule
///
/// CSS defines the At Rule as:
///
/// ```md
/// <at-rule>
///                          ╭─────────────────────────╮
///  │├─ <at-keyword-token> ─╯─╭─ <component-value> ─╮─╰─╮─ <{}-block> ─╭──┤│
///                            ╰─────────────────────╯   ╰───── ";" ────╯
/// ```
///
/// Howver a list of `<component-value>`s and a `<{}-block>` would be a very generic at-rule that would be hard to
/// reason about. AST nodes are likely to have more bespoke parsing steps (as well as denying a prelude or block) so the
/// cover grammar for this trait is better defined as:
///
/// ```md
/// <at-keyword-named>
///  │├─ <at-keyword-token NAME>
///
/// <at-rule>
///                          ╭─────────────╮
///  │├─ <at-keyword-named> ─╯─ <prelude> ─╰─ <block> ─┤│
/// ```
///
/// This simpler set of parsing steps allows AST nodes to determine the grammar for `<prelude>` ([AtRule::Prelude]) and
/// `<block>` ([AtRule::Block]) portions. Defining the const [AtRule::NAME] as `Some(&'static str)` allows the rule to
/// check the value of the `<at-keyword-token>`.
///
/// AtRules can have an optional prelude (e.g. @supoports requires one, @starting-style must not have one, and in @page
/// it is optional). Consequently `parse_at_rule` returns an [Self::Prelude] as an [Option], and rules that either
/// require should run the neccessary checks in their [Parse::parse()] implementation (or use something like
/// [NoPreludeAllowed][crate::syntax::NoPreludeAllowed] to ban a prelude).
///
/// The block is always parsed. For some rules a block is either optional or disallowed. For optional blocks, it's
/// advised to provide an `enum` for the block, which can conditionally parse the underyling block. For disallowed
/// blocks [NoBlockAllowed][crate::syntax::NoBlockAllowed] can be used to ensure that the rule only ends in `;`.
///
/// # Example
///
/// ```
/// use css_parse::*;
///
/// /// A grammar like `@test foo {}`
/// #[derive(Debug)]
/// pub struct TestAtRule<'a> {
///   pub name: T![AtKeyword],
///   pub prelude: T![Ident],
///   pub block: syntax::Block<'a>,
/// }
///
/// impl<'a> Parse<'a> for TestAtRule<'a> {
///   fn parse(p: &mut Parser<'a>) -> Result<Self> {
///     let (name, prelude, block) = Self::parse_at_rule(p)?;
///     if let Some(prelude) = prelude {
///       Ok(Self { name, prelude, block })
///     } else {
///       Err(diagnostics::MissingAtRulePrelude(name.into()))?
///     }
///   }
/// }
///
/// impl<'a> AtRule<'a> for TestAtRule<'a> {
///   const NAME: Option<&'static str> = Some("test");
///   type Block = syntax::Block<'a>;
///   type Prelude = T![Ident];
/// }
///
/// impl ToCursors for TestAtRule<'_> {
///   fn to_cursors(&self, s: &mut impl CursorSink) {
///     self.name.to_cursors(s);
///     self.prelude.to_cursors(s);
///     self.block.to_cursors(s);
///   }
/// }
///
/// assert_parse!(TestAtRule, "@test foo{}");
/// ```
///
pub trait AtRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	// Ideally Block would either implement QualifiedRuleList/DeclarationList/RuleList/DeclarationRuleList; but there is
	// no way to enforce that with Rust so it just has to implement Parse.
	type Block: Parse<'a>;

	const NAME: Option<&'static str> = None;

	fn parse_at_rule(p: &mut Parser<'a>) -> Result<(T![AtKeyword], Option<Self::Prelude>, Self::Block)> {
		let at = p.parse::<T![AtKeyword]>()?;
		if let Some(name) = Self::NAME {
			let c: Cursor = at.into();
			if !p.eq_ignore_ascii_case(c, name) {
				Err(diagnostics::UnexpectedAtRule(p.parse_str_lower(c).into(), c.into()))?;
			}
		}
		let c = p.peek_next();
		let prelude =
			if p.at_end() || c == KindSet::LEFT_CURLY_OR_SEMICOLON { None } else { Some(p.parse::<Self::Prelude>()?) };

		Ok((at, prelude, p.parse::<Self::Block>()?))
	}
}
