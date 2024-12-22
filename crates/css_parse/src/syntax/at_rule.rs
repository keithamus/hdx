use crate::{
	syntax::{Block, ComponentValues},
	AtRule as AtRuleTrait, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T,
};
use css_lexer::KindSet;

/// This struct provides the generic [`<at-rule>` grammar][1]. It will [consume an at-rule][2]. This is defined as:
///
/// ```md
/// <at-rule>
///  │├─ <at-keyword-token> ─╭─ <component-value> ─╮─╮─ <{}-block> ─╭──┤│
///                          ╰─────────────────────╯ ╰───── ";" ────╯
/// ```
///
/// A list of `<component-value>`s and a `<{}-block>` would be a very generic at-rule. AtRules that want to capture a
/// more specific grammar should instead use the [AtRule][crate::traits::AtRule] trait to specify their own grammars.
/// But this struct is useful for encountering unknown at-rules.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#at-rule-diagram
/// [2]: https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct AtRule<'a> {
	pub name: T![AtKeyword],
	pub prelude: Option<ComponentValues<'a>>,
	pub block: Block<'a>,
}

impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let stop = p.set_stop(KindSet::LEFT_CURLY_OR_SEMICOLON);
		let parsed = Self::parse_at_rule(p);
		p.set_stop(stop);
		let (name, prelude, block) = parsed?;
		Ok(Self { name, prelude, block })
	}
}

impl<'a> AtRuleTrait<'a> for AtRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl ToCursors for AtRule<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		if let Some(prelude) = &self.prelude {
			ToCursors::to_cursors(prelude, s);
		}
		ToCursors::to_cursors(&self.block, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AtRule>(), 144);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AtRule, "@foo{}");
		assert_parse!(AtRule, "@foo prelude{}");
	}
}
