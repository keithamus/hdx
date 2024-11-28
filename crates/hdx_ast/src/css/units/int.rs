use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSInt(T![Number]);

impl CSSInt {
	#[allow(non_upper_case_globals)]
	pub const Zero: CSSInt = CSSInt(<T![Number]>::ZERO);
}

impl From<CSSInt> for i32 {
	fn from(value: CSSInt) -> Self {
		value.0.into()
	}
}

impl From<CSSInt> for f32 {
	fn from(value: CSSInt) -> Self {
		value.0.into()
	}
}

impl<'a> Is<'a> for CSSInt {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::is(p, c) && c.token().is_int()
	}
}

impl<'a> Build<'a> for CSSInt {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Number]>::build(p, c))
	}
}

impl From<CSSInt> for Cursor {
	fn from(value: CSSInt) -> Self {
		value.0.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CSSInt, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CSSInt, "0");
		assert_parse!(CSSInt, "999999");
	}
}
