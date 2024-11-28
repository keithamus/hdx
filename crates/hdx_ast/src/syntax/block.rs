use bumpalo::collections::Vec;
use hdx_lexer::{SourceOffset, Token};
use hdx_parser::{Block as BlockTrait, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{Declaration, Rule};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Block<'a> {
	start: SourceOffset,
	pub open_curly: T!['{'],
	pub declarations: Vec<'a, Declaration<'a>>,
	pub rules: Vec<'a, Rule<'a>>,
	pub close_curly: Option<T!['}']>,
}

impl<'a> Parse<'a> for Block<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (open_curly, declarations, rules, close_curly) = Self::parse_block(p)?;
		Ok(Self { start, open_curly, declarations, rules, close_curly })
	}
}

impl<'a> BlockTrait<'a> for Block<'a> {
	type Declaration = Declaration<'a>;
	type Rule = Rule<'a>;
}

impl<'a> ToCursors<'a> for Block<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(Into::<Token>::into(self.open_curly).with_cursor(self.start));
		for declaration in &self.declarations {
			ToCursors::to_cursors(declaration, s);
		}
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(t) = self.close_curly {
			s.append(t.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Block, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Block, "{color:black}");
	}
}
