use crate::{CursorSink, DeclarationValue, Parse, Parser, Result, ToCursors};
use bumpalo::collections::Vec;

use super::ComponentValue;

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ComponentValues<'a> {
	values: Vec<'a, ComponentValue<'a>>,
}

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let mut values = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				break;
			}
			if p.next_is_stop() {
				break;
			}
			if p.peek::<ComponentValue>() {
				values.push(p.parse::<ComponentValue>()?);
			} else {
				break;
			}
		}
		Ok(Self { values })
	}
}

impl<'a> DeclarationValue<'a> for ComponentValues<'a> {
	fn parse_declaration_value(p: &mut Parser<'a>, _: css_lexer::Cursor) -> Result<Self> {
		Self::parse(p)
	}
}

impl<'a> ToCursors for ComponentValues<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for value in &self.values {
			ToCursors::to_cursors(value, s)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ComponentValues>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ComponentValues, "body{color:black}");
		assert_parse!(ComponentValues, "body");
	}
}
