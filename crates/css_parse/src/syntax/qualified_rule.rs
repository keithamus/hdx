use crate::{
	syntax::{BadDeclaration, Block, ComponentValues},
	CursorSink, Parse, Parser, QualifiedRule as QualifiedRuleTrait, Result, ToCursors,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a> {
	pub prelude: ComponentValues<'a>,
	pub block: Block<'a>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
impl<'a> Parse<'a> for QualifiedRule<'a> {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let (prelude, block) = Self::parse_qualified_rule(p)?;
		Ok(Self { prelude, block })
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
	type BadDeclaration = BadDeclaration<'a>;
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
		assert_eq!(std::mem::size_of::<QualifiedRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(QualifiedRule, "body{color:black}");
	}
}
