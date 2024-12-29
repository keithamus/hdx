use css_lexer::{Cursor, DimensionUnit};
use css_parse::{Build, Parser, Peek, T};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CustomDimension(T![Dimension]);

impl From<CustomDimension> for f32 {
	fn from(custom: CustomDimension) -> Self {
		custom.0.into()
	}
}

impl<'a> Peek<'a> for CustomDimension {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && c == DimensionUnit::Unknown && p.parse_str(c).starts_with("--")
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CustomDimension>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CustomDimension, "1--foo");
	}
}
