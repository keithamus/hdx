use hdx_lexer::{Cursor, DimensionUnit};
use hdx_parser::{Build, Is, Parser, T};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CustomDimension(T![Dimension]);

impl From<CustomDimension> for f32 {
	fn from(custom: CustomDimension) -> Self {
		custom.0.into()
	}
}

impl<'a> Is<'a> for CustomDimension {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::is(p, c) && c == DimensionUnit::Unknown && p.parse_atom(c).starts_with("--")
	}
}

impl<'a> Build<'a> for CustomDimension {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Dimension]>::build(p, c))
	}
}

impl From<CustomDimension> for Cursor {
	fn from(value: CustomDimension) -> Self {
		value.0.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CustomDimension, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CustomDimension, "1--foo");
	}
}
