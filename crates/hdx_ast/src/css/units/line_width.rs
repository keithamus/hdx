use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

use super::Length;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Thin, atom!("thin"));
	custom_keyword!(Medium, atom!("medium"));
	custom_keyword!(Thick, atom!("thick"));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LineWidth {
	Thin(kw::Thin),
	Medium(kw::Medium),
	Thick(kw::Thick),
	Length(Length),
}

impl<'a> Is<'a> for LineWidth {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		Length::is(p, c)
			|| (<T![Ident]>::is(p, c)
				&& matches!(p.parse_atom_lower(c), atom!("thin") | atom!("medium") | atom!("thick")))
	}
}

impl<'a> Build<'a> for LineWidth {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if Length::is(p, c) {
			Self::Length(Length::build(p, c))
		} else {
			match p.parse_atom_lower(c) {
				atom!("medium") => Self::Medium(kw::Medium::build(p, c)),
				atom!("thin") => Self::Thin(kw::Thin::build(p, c)),
				atom!("thick") => Self::Thick(kw::Thick::build(p, c)),
				_ => unreachable!(),
			}
		}
	}
}

impl From<LineWidth> for Cursor {
	fn from(value: LineWidth) -> Self {
		match value {
			LineWidth::Thin(t) => t.into(),
			LineWidth::Medium(t) => t.into(),
			LineWidth::Thick(t) => t.into(),
			LineWidth::Length(t) => t.into(),
		}
	}
}

// impl From<LineWidth> for Length {
// 	fn from(value: LineWidth) -> Self {
// 		match value {
// 			LineWidth::Thin => Length::Px(1.0.into()),
// 			LineWidth::Medium => Length::Px(3.0.into()),
// 			LineWidth::Thick => Length::Px(3.0.into()),
// 			LineWidth::Length(length) => length,
// 		}
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(LineWidth, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LineWidth, "1px");
		assert_parse!(LineWidth, "medium");
	}
}
