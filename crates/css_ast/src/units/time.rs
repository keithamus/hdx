use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

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

impl<'a> Peek<'a> for Time {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
			|| <T![Dimension]>::peek(p, c) && matches!(p.parse_str_lower(c), "s" | "ms")
	}
}

impl<'a> Build<'a> for Time {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::peek(p, c) && c.token().value() == 0.0 {
			Self::Zero(<T![Number]>::build(p, c))
		} else {
			match p.parse_str_lower(c) {
				"s" => Self::S(<T![Dimension::S]>::build(p, c)),
				"ms" => Self::Ms(<T![Dimension::Ms]>::build(p, c)),
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Time>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Time, "0s");
	}
}
