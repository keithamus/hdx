use crate::{syntax::ComponentValue, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use bumpalo::collections::Vec;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct FunctionBlock<'a> {
	pub name: T![Function],
	pub values: Vec<'a, ComponentValue<'a>>,
	pub close_paren: Option<T![')']>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for FunctionBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let name = p.parse::<T![Function]>()?;
		let mut values = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				break;
			}
			if p.peek::<T![')']>() {
				break;
			}
			values.push(p.parse::<ComponentValue>()?);
		}
		Ok(Self { name, values, close_paren: p.parse::<T![')']>().ok() })
	}
}

impl<'a> ToCursors for FunctionBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		for value in &self.values {
			ToCursors::to_cursors(value, s);
		}
		if let Some(close) = self.close_paren {
			s.append(close.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FunctionBlock, "foo(bar)");
		assert_parse!(FunctionBlock, "foo(bar{})");
	}
}
