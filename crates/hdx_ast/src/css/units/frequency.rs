use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Frequency {
	Hz(T![Dimension::Hz]),
	Khz(T![Dimension::Khz]),
}

impl Default for Frequency {
	fn default() -> Self {
		Self::Hz(Default::default())
	}
}

impl From<Frequency> for f32 {
	fn from(frequency: Frequency) -> Self {
		match frequency {
			Frequency::Hz(f) => f.into(),
			Frequency::Khz(f) => f.into(),
		}
	}
}

impl<'a> Is<'a> for Frequency {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::is(p, c) && matches!(p.parse_atom_lower(c), atom!("hz") | atom!("khz"))
	}
}

impl<'a> Build<'a> for Frequency {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("hz") => Self::Hz(<T![Dimension::Hz]>::build(p, c)),
			atom!("khz") => Self::Khz(<T![Dimension::Khz]>::build(p, c)),
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
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Frequency, 12);
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
