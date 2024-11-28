use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSFloat(T![Number]);

impl CSSFloat {
	#[allow(non_upper_case_globals)]
	pub const Zero: CSSFloat = CSSFloat(<T![Number]>::ZERO);
}

impl From<CSSFloat> for i32 {
	fn from(value: CSSFloat) -> Self {
		value.0.into()
	}
}

impl From<CSSFloat> for f32 {
	fn from(value: CSSFloat) -> Self {
		value.0.into()
	}
}

impl<'a> Is<'a> for CSSFloat {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::is(p, c) && c.token().is_float()
	}
}

impl<'a> Build<'a> for CSSFloat {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Number]>::build(p, c))
	}
}

impl From<CSSFloat> for Cursor {
	fn from(value: CSSFloat) -> Self {
		value.0.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CSSFloat, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CSSFloat, "0.01");
		assert_parse!(CSSFloat, "3.141");
	}
}
