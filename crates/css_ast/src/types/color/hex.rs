use css_parse::{Cursor, Diagnostic, Kind, KindSet, Parse, Parser, Peek, Result as ParserResult};
use csskit_derives::*;
use csskit_proc_macro::node;

/// `<hex-color>` as defined in [css-color-4](https://drafts.csswg.org/css-color-4/#hex-notation).
///
/// A hash token of exactly 3, 4, 6 or 8 hexadecimal digits.
#[node]
#[derive(IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub struct HexColor(Cursor);

impl HexColor {
	/// Returns the colour as `0xRRGGBBAA`.
	pub fn hex_value(&self) -> u32 {
		self.0.token().hex_value()
	}
}

impl<'a> Peek<'a> for HexColor {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Hash]);

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if c != Kind::Hash {
			return false;
		}
		let token = c.token();
		match token.len() {
			4 | 7 => token.hex_value() != 0,
			5 | 9 => token.hex_value() != 0 || c.str_slice(p.source_text()).bytes().skip(1).all(|b| b == b'0'),
			_ => false,
		}
	}
}

impl<'a> Parse<'a> for HexColor {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) { Ok(Self(c)) } else { Err(Diagnostic::new(c, Diagnostic::unexpected))? }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#fff");
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#ffff");
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#ababab");
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#abababab");
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#0000");
		assert_parse!(CssAtomSet::ATOMS, HexColor, "#00000000");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, HexColor, "#F0000");
		assert_peek_false!(CssAtomSet::ATOMS, HexColor, "#ff");
		assert_peek_false!(CssAtomSet::ATOMS, HexColor, "#fffffff");
		assert_peek_false!(CssAtomSet::ATOMS, HexColor, "#ggg");
		assert_peek_false!(CssAtomSet::ATOMS, HexColor, "#gggg");
	}
}
