use core::fmt;

use bumpalo::{
	collections::{vec::IntoIter, Vec},
	Bump,
};
use hdx_atom::Atomizable;
use hdx_lexer::{Cursor, DimensionUnit, Kind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorStream<'a> {
	cursors: Vec<'a, Cursor>,
}

impl<'a> CursorStream<'a> {
	pub fn new(allocator: &'a Bump) -> Self {
		Self { cursors: Vec::new_in(allocator) }
	}

	pub fn append(&mut self, c: Cursor) {
		self.cursors.push(c);
	}

	// https://drafts.csswg.org/css-syntax-3/#serialization
	pub fn write(&self, str: &'a str, f: &mut impl fmt::Write) -> fmt::Result {
		let mut last_kind: Kind = Kind::Eof;
		for c in self.cursors.iter() {
			let t = c.token();
			let kind = t.kind();
			match kind {
				Kind::Eof => {}
				Kind::Whitespace => {
					for _ in 0..c.token().len() {
						f.write_str(t.whitespace_style().as_str())?;
					}
				}
				Kind::Comment => f.write_str(c.str_slice(str))?,
				Kind::CdcOrCdo => {
					if t.is_cdc() {
						f.write_str("-->")?
					} else {
						f.write_str("<!--")?
					}
				}
				Kind::Number => {
					if t.has_sign() {
						write!(f, "{:+}", t.value())?;
					} else {
						if last_kind.ambiguous_without_whitespace() {
							f.write_char(' ')?;
						}
						write!(f, "{}", t.value())?;
					}
				}
				Kind::Dimension => {
					if last_kind.ambiguous_without_whitespace() {
						f.write_char(' ')?;
					}
					match c.token().dimension_unit() {
						DimensionUnit::Unknown => f.write_str(c.str_slice(str))?,
						d => {
							f.write_str(&c.token().value().to_string())?;
							f.write_str(&d.to_atom())?;
						}
					}
				}
				Kind::BadString => f.write_str(c.str_slice(str))?,
				Kind::BadUrl => f.write_str(c.str_slice(str))?,
				Kind::Ident => {
					if last_kind.ambiguous_without_whitespace() {
						f.write_char(' ')?;
					}
					f.write_str(c.str_slice(str))?;
				}
				Kind::Function => f.write_str(c.str_slice(str))?,
				Kind::AtKeyword => f.write_str(c.str_slice(str))?,
				Kind::Hash => f.write_str(c.str_slice(str))?,
				Kind::String => f.write_str(c.str_slice(str))?,
				Kind::Url => f.write_str(c.str_slice(str))?,
				Kind::Delim
				| Kind::Colon
				| Kind::Semicolon
				| Kind::Comma
				| Kind::LeftSquare
				| Kind::RightSquare
				| Kind::LeftParen
				| Kind::RightParen
				| Kind::LeftCurly
				| Kind::RightCurly => f.write_char(t.char().unwrap())?,
			}
			last_kind = kind;
		}
		Ok(())
	}
}

impl<'a> IntoIterator for CursorStream<'a> {
	type Item = Cursor;

	type IntoIter = IntoIter<'a, Cursor>;

	fn into_iter(self) -> Self::IntoIter {
		self.cursors.into_iter()
	}
}
