use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Time {
	Zero(T![Number]),
	Ms(T![Dimension::Ms]),
	S(T![Dimension::S]),
}

impl From<Time> for f32 {
	fn from(val: Time) -> Self {
		match val {
			Time::Zero(_) => 0.0,
			Time::Ms(f) => f.into(),
			Time::S(f) => f.into(),
		}
	}
}

impl<'a> Is<'a> for Time {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::is(p, c) && c.token().value() == 0.0)
			|| <T![Dimension]>::is(p, c) && matches!(p.parse_atom_lower(c), atom!("s") | atom!("ms"))
	}
}

impl<'a> Build<'a> for Time {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::is(p, c) && c.token().value() == 0.0 {
			Self::Zero(<T![Number]>::build(p, c))
		} else {
			match p.parse_atom_lower(c) {
				atom!("s") => Self::S(<T![Dimension::S]>::build(p, c)),
				atom!("ms") => Self::Ms(<T![Dimension::Ms]>::build(p, c)),
				_ => unreachable!(),
			}
		}
	}
}

impl From<Time> for Cursor {
	fn from(value: Time) -> Self {
		match value {
			Time::Zero(t) => t.into(),
			Time::Ms(t) => t.into(),
			Time::S(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Time, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Time, "0s");
	}
}
