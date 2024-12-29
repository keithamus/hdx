use css_lexer::{Cursor, Token};
use css_parse::{Build, Parser, Peek, T};

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Flex(T![Dimension::Fr]);

impl From<Flex> for f32 {
	fn from(flex: Flex) -> Self {
		flex.0.into()
	}
}

impl<'a> Peek<'a> for Flex {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension::Fr]>::peek(p, c)
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Flex>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Flex, "1fr");
	}
}
