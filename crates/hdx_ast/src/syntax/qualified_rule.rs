use hdx_parser::{CursorSink, Parse, Parser, QualifiedRule as QualifiedRuleTrait, Result as ParserResult, ToCursors};

use super::{BadDeclaration, Block, ComponentValues};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a> {
	pub prelude: ComponentValues<'a>,
	pub block: Block<'a>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
impl<'a> Parse<'a> for QualifiedRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (prelude, block) = Self::parse_qualified_rule(p)?;
		Ok(Self { prelude, block })
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
	type BadDeclaration = BadDeclaration;
}

impl<'a> ToCursors for QualifiedRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.prelude, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(QualifiedRule, 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(QualifiedRule, "body{color:black}");
	}
}
