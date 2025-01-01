use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

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

impl<'a> Peek<'a> for Resolution {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && matches!(p.parse_str_lower(c), "dpi" | "dpcm" | "dppx")
	}
}

impl<'a> Build<'a> for Resolution {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_str_lower(c) {
			"dpi" => Self::Dpi(<T![Dimension::Dpi]>::build(p, c)),
			"dpcm" => Self::Dpcm(<T![Dimension::Dpcm]>::build(p, c)),
			"dppx" => Self::Dppx(<T![Dimension::Dppx]>::build(p, c)),
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Resolution>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Resolution, "1dppx");
	}
}
