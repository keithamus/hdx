use crate::{diagnostics, keyword_set, Build, Parse, Parser, Peek, Result};
use bumpalo::collections::Vec;

keyword_set!(ConditionKeyword { And: "and", Not: "not", Or: "or" });

/// This trait can be used for AST nodes representing a list of "Feature Conditions". This is an amalgamation of
/// [Supports Conditions][1], [Media Conditions][2], and [Container Queries][3]
/// This is an implementation of [`<at-rule-list>`][1].
///
/// Looking at `<supports-condition> and `<container-query>` we can se almost identical grammars (eliding some tokens
/// for brevity):
///
/// ```md
/// <supports-condition>
///  │├─╮─ <ident-token "not"> ─ <supports-in-parens> ──────────────────────────────╭──┤│
///     ╰─ <supports-in-parens> ─╮─╭─ <ident-token "and"> ─ <supports-in-parens> ─╮─┤
///                              │ ╰──────────────────────────────────────────────╯ │
///                              ├─╭─ <ident-token "or"> ─ <supports-in-parens> ─╮──┤
///                              │ ╰─────────────────────────────────────────────╯  │
///                              ╰──────────────────────────────────────────────────╯
///                               
/// <container-query>
///  │├─╮─ <ident-token "not"> ─ <query-in-parens> ───────────────────────────╭──┤│
///     ╰─ <supports-in-parens> ─╮─╭─ <ident-token "and"> ─ <supports-in-parens> ─╮─┤
///                              │ ╰──────────────────────────────────────────────╯ │
///                              ├─╭─ <ident-token "or"> ─ <supports-in-parens> ─╮──┤
///                              │ ╰─────────────────────────────────────────────╯  │
///                              ╰──────────────────────────────────────────────────╯
///
/// <media-condition>
///  │├─╮─ <ident-token "not"> ─ <media-in-parens> ───────────────────────────╭──┤│
///     ╰─ <media-in-parens> ─╮─╭─ <ident-token "and"> ─ <media-in-parens> ─╮─┤
///                           │ ╰───────────────────────────────────────────╯ │
///                           │─╭─ <ident-token "or"> ─ <media-in-parens> ─╮──│
///                           │ ╰──────────────────────────────────────────╯  │
///                           ╰───────────────────────────────────────────────╯
/// ```
///
/// The key difference between each of these is their own `<*-in-parens>` tokens. Thus they could all be defined as:
///
/// ```md
/// <condition-prelude-list>
///  │├─╮─ <ident-token "not"> ─ <feature> ───────────────────╭──┤│
///     ╰─ <feature> ─╮─╭─ <ident-token "and"> ─ <feature> ─╮─┤
///                   │ ╰───────────────────────────────────╯ │
///                   │─╭─ <ident-token "or"> ─ <feature> ─╮──│
///                   │ ╰──────────────────────────────────╯  │
///                   ╰───────────────────────────────────────╯
/// ```
///
/// Where `<feature>` is defined by `[FeatureConditionList::FeatureCondition]`, which is required to implement [Parse].
/// There is a further subtle change for this trait, which is the introduction of the [ConditionKeyword] enum to better
/// reason about the given condition keyword. This makes the final grammar:
///
/// ```md
///	<condition-keyword>
///  │├──╮─ <ident-token "not"> ─╭──┤│
///      ├─ <ident-token "and"> ─┤
///      ╰─ <ident-token "or"> ──╯
///
/// <condition-prelude-list>
///  │├─╮─ <condition-keyword "not"> ─ <feature> ───────────────────╭──┤│
///     ╰─ <feature> ─╮─╭─ <condition-keyword "and"> ─ <feature> ─╮─┤
///                   │ ╰─────────────────────────────────────────╯ │
///                   │─╭─ <condition-keyword "or"> ─ <feature> ─╮──│
///                   │ ╰────────────────────────────────────────╯  │
///                   ╰─────────────────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-conditional-3/#typedef-supports-condition
/// [2]: https://drafts.csswg.org/mediaqueries/#media-condition
/// [3]: https://drafts.csswg.org/css-conditional-5/#typedef-container-query
pub trait FeatureConditionList<'a>: Sized + Parse<'a>
where
	Self: 'a,
{
	type FeatureCondition: Sized + Parse<'a>;

	fn build_is(feature: Self::FeatureCondition) -> Self;
	fn build_not(keyword: ConditionKeyword, features: Self::FeatureCondition) -> Self;
	fn build_and(features: Vec<'a, (Self::FeatureCondition, Option<ConditionKeyword>)>) -> Self;
	fn build_or(features: Vec<'a, (Self::FeatureCondition, Option<ConditionKeyword>)>) -> Self;

	fn parse_condition(p: &mut Parser<'a>) -> Result<Self> {
		let c = p.peek_next();
		if ConditionKeyword::peek(p, c) {
			let keyword = ConditionKeyword::build(p, c);
			if matches!(keyword, ConditionKeyword::Not(_)) {
				return Ok(Self::build_is(p.parse::<Self::FeatureCondition>()?));
			}
			Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
		}
		let mut feature = p.parse::<Self::FeatureCondition>()?;
		let keyword = p.parse_if_peek::<ConditionKeyword>()?;
		match keyword {
			Some(ConditionKeyword::And(_)) => {
				let mut features = Vec::new_in(p.bump());
				let mut keyword = keyword.unwrap();
				loop {
					features.push((feature, Some(keyword)));
					feature = p.parse::<Self::FeatureCondition>()?;
					let c = p.peek_next();
					if !ConditionKeyword::peek(p, c) || !matches!(ConditionKeyword::build(p, c), ConditionKeyword::And(_)) {
						features.push((feature, None));
						return Ok(Self::build_and(features));
					}
					keyword = ConditionKeyword::build(p, c);
				}
			},
			Some(ConditionKeyword::Or(_)) => {
				let mut features = Vec::new_in(p.bump());
				let mut keyword = keyword.unwrap();
				loop {
					features.push((feature, Some(keyword)));
					feature = p.parse::<Self::FeatureCondition>()?;
					let c = p.peek_next();
					if !ConditionKeyword::peek(p, c) || !matches!(ConditionKeyword::build(p, c), ConditionKeyword::And(_)) {
						features.push((feature, None));
						return Ok(Self::build_or(features));
					}
					keyword = ConditionKeyword::build(p, c);
				}
			},
			Some(ConditionKeyword::Not(_)) => Ok(Self::build_not(keyword.unwrap(), p.parse::<Self::FeatureCondition>()?)),
			None => Ok(Self::build_is(feature)),
		}
	}
}
