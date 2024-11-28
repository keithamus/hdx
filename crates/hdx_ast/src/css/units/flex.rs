use hdx_lexer::{Cursor, Token};
use hdx_parser::{Build, Is, Parser, T};

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Flex(T![Dimension::Fr]);

impl From<Flex> for f32 {
	fn from(flex: Flex) -> Self {
		flex.0.into()
	}
}

impl<'a> Is<'a> for Flex {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension::Fr]>::is(p, c)
	}
}

impl<'a> Build<'a> for Flex {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Dimension::Fr]>::build(p, c))
	}
}

impl From<Flex> for Token {
	fn from(flex: Flex) -> Self {
		flex.0.into()
	}
}

impl From<Flex> for Cursor {
	fn from(flex: Flex) -> Self {
		flex.0.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Flex, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Flex, "1fr");
	}
}
