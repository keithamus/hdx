use hdx_lexer::{Cursor, DimensionUnit};
use hdx_parser::{Build, Is, Parser, T};

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Custom(T![Dimension]);

impl From<Custom> for f32 {
	fn from(custom: Custom) -> Self {
		custom.0.into()
	}
}

impl<'a> Is<'a> for Custom {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::is(p, c) && c == DimensionUnit::Unknown && p.parse_atom(c).starts_with("--")
	}
}

impl<'a> Build<'a> for Custom {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Dimension]>::build(p, c))
	}
}

impl From<Custom> for Cursor {
	fn from(value: Custom) -> Self {
		value.0.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Custom, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Custom, "1--foo");
	}
}
