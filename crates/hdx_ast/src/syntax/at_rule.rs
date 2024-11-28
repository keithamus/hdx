use hdx_parser::{AtRule as AtRuleTrait, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{Block, ComponentValues};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct AtRule<'a> {
	pub name: T![AtKeyword],
	pub prelude: Option<ComponentValues<'a>>,
	pub block: Option<Block<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, prelude, block) = Self::parse_at_rule(p, None)?;
		Ok(Self { name, prelude, block })
	}
}

impl<'a> AtRuleTrait<'a> for AtRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> ToCursors<'a> for AtRule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.name.into());
		if let Some(prelude) = &self.prelude {
			ToCursors::to_cursors(prelude, s);
		}
		if let Some(block) = &self.block {
			ToCursors::to_cursors(block, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AtRule, 136);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AtRule, "@foo");
		assert_parse!(AtRule, "@foo prelude;");
		assert_parse!(AtRule, "@foo prelude{};");
	}
}
