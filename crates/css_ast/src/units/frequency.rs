use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Frequency {
	Hz(T![Dimension::Hz]),
	Khz(T![Dimension::Khz]),
}

impl From<Frequency> for f32 {
	fn from(frequency: Frequency) -> Self {
		match frequency {
			Frequency::Hz(f) => f.into(),
			Frequency::Khz(f) => f.into(),
		}
	}
}

impl<'a> Peek<'a> for Frequency {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && matches!(p.parse_str_lower(c), "hz" | "khz")
	}
}

impl<'a> Build<'a> for Frequency {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_str_lower(c) {
			"hz" => Self::Hz(<T![Dimension::Hz]>::build(p, c)),
			"khz" => Self::Khz(<T![Dimension::Khz]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<Frequency> for Cursor {
	fn from(value: Frequency) -> Self {
		match value {
			Frequency::Hz(t) => t.into(),
			Frequency::Khz(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Frequency>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Frequency, "40hz");
		assert_parse!(Frequency, "40khz");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Frequency, "40w");
		assert_parse_error!(Frequency, "40kw");
	}
}
