use bumpalo::collections::Vec;
use hdx_parser::{CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::ComponentValue;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: T![Function],
	pub values: Vec<'a, ComponentValue<'a>>,
	pub close_paren: Option<T![')']>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
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

impl<'a> ToCursors for Function<'a> {
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
		assert_size!(Function, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Function, "foo(bar)");
		assert_parse!(Function, "foo(bar{})");
	}
}
