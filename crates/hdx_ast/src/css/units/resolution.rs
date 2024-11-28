use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

const DPPX_IN: f32 = 96.0;
const DPPX_CM: f32 = DPPX_IN / 2.54;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	Dpi(T![Dimension::Dpi]),
	Dpcm(T![Dimension::Dpcm]),
	Dppx(T![Dimension::Dppx]),
}

impl Default for Resolution {
	fn default() -> Self {
		Self::Dppx(Default::default())
	}
}

impl From<Resolution> for f32 {
	fn from(res: Resolution) -> Self {
		match res {
			Resolution::Dpi(r) => r.into(),
			Resolution::Dpcm(r) => r.into(),
			Resolution::Dppx(r) => r.into(),
		}
	}
}

impl From<&Resolution> for f32 {
	fn from(res: &Resolution) -> Self {
		match res {
			Resolution::Dpi(r) => r.into(),
			Resolution::Dpcm(r) => r.into(),
			Resolution::Dppx(r) => r.into(),
		}
	}
}

impl<'a> Is<'a> for Resolution {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::is(p, c) && matches!(p.parse_atom_lower(c), atom!("dpi") | atom!("dpcm") | atom!("dppx"))
	}
}

impl<'a> Build<'a> for Resolution {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("dpi") => Self::Dpi(<T![Dimension::Dpi]>::build(p, c)),
			atom!("dpcm") => Self::Dpcm(<T![Dimension::Dpcm]>::build(p, c)),
			atom!("dppx") => Self::Dppx(<T![Dimension::Dppx]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<Resolution> for Cursor {
	fn from(value: Resolution) -> Self {
		match value {
			Resolution::Dpi(t) => t.into(),
			Resolution::Dpcm(t) => t.into(),
			Resolution::Dppx(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Resolution, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Resolution, "1dppx");
	}
}
