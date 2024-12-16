use std::{char::REPLACEMENT_CHARACTER, fmt};

use bumpalo::{collections::String, Bump};
use hdx_atom::{Atom, Atomizable};
use hdx_syntax::{is_escape_sequence, is_newline, is_whitespace, ParseEscape, EOF};

use crate::{span::SpanContents, CommentStyle, DimensionUnit, Kind, KindSet, QuoteStyle, SourceOffset, Span, Token};

// The `Cursor` type is a wrapping of the immutable `Token`, plus an offset
// into a text document (&'a str). The Cursor's knowledge of the underlying
// text lives inside `Token`.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor(SourceOffset, Token);

impl Cursor {
	pub const DUMMY_SITE_NUMBER_ZERO: Cursor = Cursor(SourceOffset(0), Token::NUMBER_ZERO);

	#[inline(always)]
	pub const fn new(offset: SourceOffset, token: Token) -> Self {
		Self(offset, token)
	}

	#[inline(always)]
	pub const fn dummy(token: Token) -> Self {
		Self(SourceOffset::DUMMY, token)
	}

	#[inline(always)]
	pub const fn offset(&self) -> SourceOffset {
		self.0
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.1
	}

	#[inline(always)]
	pub fn end_offset(&self) -> SourceOffset {
		if self.offset() == SourceOffset::DUMMY {
			return self.offset();
		}
		SourceOffset(self.offset().0 + self.len())
	}

	#[inline(always)]
	pub const fn is_empty(&self) -> bool {
		self.token().is_empty()
	}

	#[inline(always)]
	pub fn len(&self) -> u32 {
		self.token().len()
	}

	#[inline(always)]
	pub fn span(&self) -> Span {
		Span::new(self.offset(), self.end_offset())
	}

	pub fn write_str(&self, str: &str, f: &mut impl fmt::Write) -> fmt::Result {
		match self.token().kind() {
			Kind::Eof => {}
			Kind::Whitespace => {
				for _ in 0..self.token().len() {
					f.write_str(self.token().whitespace_style().as_str())?;
				}
			}
			Kind::CdcOrCdo => {
				if self.token().is_cdc() {
					f.write_str("-->")?
				} else {
					f.write_str("<!--")?
				}
			}
			Kind::Number => {
				if self.token().has_sign() {
					write!(f, "{:+}", self.token().value())?;
				} else {
					write!(f, "{}", self.token().value())?;
				}
			}
			Kind::Dimension => match self.token().dimension_unit() {
				DimensionUnit::Unknown => f.write_str(self.str_slice(str))?,
				d => {
					f.write_str(&self.token().value().to_string())?;
					f.write_str(&d.to_atom())?;
				}
			},
			Kind::Comment
			| Kind::BadString
			| Kind::BadUrl
			| Kind::Ident
			| Kind::Function
			| Kind::AtKeyword
			| Kind::Hash
			| Kind::String
			| Kind::Url => f.write_str(self.str_slice(str))?,
			Kind::Delim
			| Kind::Colon
			| Kind::Semicolon
			| Kind::Comma
			| Kind::LeftSquare
			| Kind::LeftParen
			| Kind::RightSquare
			| Kind::RightParen
			| Kind::LeftCurly
			| Kind::RightCurly => f.write_char(self.token().char().unwrap())?,
		}
		Ok(())
	}

	#[inline(always)]
	pub fn span_contents<'a>(&self, str: &'a str) -> SpanContents<'a> {
		self.span().span_contents(str)
	}

	#[inline(always)]
	pub fn str_slice<'a>(&self, str: &'a str) -> &'a str {
		self.span_contents(str).contents()
	}

	pub fn eq_ignore_ascii_case<'a>(&self, source: &'a str, other: &'a str) -> bool {
		debug_assert!(self != Kind::Delim && self != Kind::Url);
		debug_assert!(other.to_ascii_lowercase() == other);
		let kind = self.token().kind();
		let start = self.offset().0 as usize
			+ match kind {
				Kind::AtKeyword | Kind::Hash | Kind::String => 1,
				Kind::Dimension => self.token().numeric_len() as usize,
				Kind::Comment => 2,
				_ => 0,
			};
		let end = self.end_offset().0 as usize
			- match kind {
				Kind::Function => 1,
				Kind::String if self.token().string_has_closing_quote() => 1,
				Kind::Comment if self.token().comment_style().unwrap().is_block() => 2,
				_ => 0,
			};
		if !self.token().contains_escape_chars() {
			if end - start != other.len() {
				return false;
			}
			if self.token().is_lower_case() {
				debug_assert!(source[start..end].to_ascii_lowercase() == source[start..end]);
				return &source[start..end] == other;
			}
			return source[start..end].eq_ignore_ascii_case(other);
		}
		let mut chars = source[start..end].chars().peekable();
		let mut other_chars = other.chars();
		let mut i = 0;
		while let Some(c) = chars.next() {
			let o = other_chars.next();
			if o.is_none() {
				return false;
			}
			let o = o.unwrap();
			if c == '\0' {
				if REPLACEMENT_CHARACTER != o {
					return false;
				}
				i += 1;
			} else if c == '\\' {
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-cursor
				if self.token().kind_bits() == Kind::String as u8 {
					// When the token is a string, escaped EOF points are not consumed
					// U+005C REVERSE SOLIDUS (\)
					//   If the next input code point is EOF, do nothing.
					//   Otherwise, if the next input code point is a newline, consume it.
					let c = chars.peek();
					if let Some(c) = c {
						if is_newline(*c) {
							chars.next();
							if chars.peek() == Some(&'\n') {
								i += 1;
							}
							i += 2;
							chars = source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = source[(start + i)..].chars().parse_escape_sequence();
				i += n as usize;
				chars = source[(start + i)..end].chars().peekable();
				if (ch == '\0' && REPLACEMENT_CHARACTER != o) || ch != o {
					return false;
				}
			} else if c != o {
				return false;
			} else {
				i += c.len_utf8();
			}
		}
		other_chars.next().is_none()
	}

	pub fn parse_str<'a>(&self, source: &'a str, allocator: &'a Bump) -> &'a str {
		debug_assert!(self != Kind::Delim);
		let kind = self.token().kind();
		if kind == Kind::Url {
			return self.parse_url_str(source, allocator);
		}
		let start = self.offset().0 as usize
			+ match kind {
				Kind::AtKeyword | Kind::Hash | Kind::String => 1,
				Kind::Dimension => self.token().numeric_len() as usize,
				Kind::Comment => 2,
				_ => 0,
			};
		let end = self.end_offset().0 as usize
			- match kind {
				Kind::Function => 1,
				Kind::String if self.token().string_has_closing_quote() => 1,
				Kind::Comment if self.token().comment_style().unwrap().is_block() => 2,
				_ => 0,
			};
		if !self.token().contains_escape_chars() {
			return &source[start..end];
		}
		let mut chars = source[start..end].chars().peekable();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		while let Some(c) = chars.next() {
			if c == '\0' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::from_str_in("", allocator))
					} else {
						Some(String::from_str_in(&source[start..(start + i)], allocator))
					}
				}
				str.as_mut().unwrap().push(REPLACEMENT_CHARACTER);
				i += 1;
			} else if c == '\\' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::from_str_in("", allocator))
					} else {
						Some(String::from_str_in(&source[start..(start + i)], allocator))
					}
				}
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-cursor
				if self.token().kind_bits() == Kind::String as u8 {
					// When the token is a string, escaped EOF points are not consumed
					// U+005C REVERSE SOLIDUS (\)
					//   If the next input code point is EOF, do nothing.
					//   Otherwise, if the next input code point is a newline, consume it.
					let c = chars.peek();
					if let Some(c) = c {
						if is_newline(*c) {
							chars.next();
							if chars.peek() == Some(&'\n') {
								i += 1;
							}
							i += 2;
							chars = source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = source[(start + i)..].chars().parse_escape_sequence();
				str.as_mut().unwrap().push(if ch == '\0' { REPLACEMENT_CHARACTER } else { ch });
				i += n as usize;
				chars = source[(start + i)..end].chars().peekable();
			} else {
				if let Some(text) = &mut str {
					text.push(c);
				}
				i += c.len_utf8();
			}
		}
		if str.is_some() {
			str.take().unwrap().into_bump_str()
		} else {
			&source[start..start + i]
		}
	}

	fn parse_url_str<'a>(&self, source: &'a str, allocator: &'a Bump) -> &'a str {
		debug_assert!(self == Kind::Url);
		// Url is special because we need to factor in that the function identifier itself can be escaped;
		let mut off = if self.token().contains_escape_chars() {
			let mut chars = source[self.offset().0 as usize..].chars().peekable();
			let mut i = 0;
			while let Some(c) = chars.next() {
				if c == '(' {
					i += 1;
					break;
				} else if is_escape_sequence(c, *chars.peek().unwrap_or(&EOF)) {
					let (_, n) = source[self.offset().0 as usize + i..].chars().parse_escape_sequence();
					i += n as usize;
				} else {
					i += 1;
				}
			}
			i
		} else {
			4
		};
		if self.token().url_has_leading_space() {
			// Url is also special because we need to remove leading whitespace...
			let mut chars = source[self.offset().0 as usize + off..].chars();
			while is_whitespace(chars.next().unwrap_or(EOF)) {
				off += 1;
			}
		}
		let start = self.offset().0 as usize + off;
		let end = self.end_offset().0 as usize - (self.token().url_has_closing_paren() as usize);
		if self.token().can_escape() && !self.token().contains_escape_chars() {
			return &source[start..end];
		}
		let mut chars = source[start..end].chars();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		while let Some(c) = chars.next() {
			match c {
				c if c == ')' || is_whitespace(c) => {
					break;
				}
				'\\' => {
					if str.is_none() {
						str = if i == 0 {
							Some(String::from_str_in("", allocator))
						} else {
							Some(String::from_str_in(&source[start..(start + i)], allocator))
						}
					}
					i += 1;
					let (ch, n) = source[start + i..].chars().parse_escape_sequence();
					if is_newline(c) && source[(start + i + (n as usize))..].starts_with('\n') {
						i += 1;
					}
					str.as_mut().unwrap().push(ch);
					i += n as usize;
					chars = source[(start + i)..end].chars();
				}
				c => {
					if let Some(text) = &mut str {
						text.push(c);
					}
					i += c.len_utf8();
				}
			}
		}
		if str.is_some() {
			str.take().unwrap().into_bump_str()
		} else {
			&source[start..start + i]
		}
	}

	#[inline]
	pub fn parse_atom<'a>(&self, source: &'a str, allocator: &'a Bump) -> Atom {
		Atom::from(self.parse_str(source, allocator))
	}

	#[inline]
	pub fn parse_atom_lower<'a>(&self, source: &'a str, allocator: &'a Bump) -> Atom {
		if self == Kind::Dimension {
			let unit = self.token().dimension_unit();
			if unit != DimensionUnit::Unknown {
				return unit.to_atom();
			}
		}
		let str = self.parse_str(source, allocator);
		if self.token().is_lower_case() {
			return Atom::from(str);
		}
		Atom::from(str.to_ascii_lowercase())
	}
}

impl From<Cursor> for Token {
	fn from(cursor: Cursor) -> Self {
		cursor.token()
	}
}

impl PartialEq<Token> for Cursor {
	fn eq(&self, other: &Token) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for Span {
	fn from(cursor: Cursor) -> Self {
		cursor.span()
	}
}

impl From<&Cursor> for Span {
	fn from(cursor: &Cursor) -> Self {
		cursor.span()
	}
}

impl PartialEq<Span> for Cursor {
	fn eq(&self, other: &Span) -> bool {
		self.span() == *other
	}
}

impl From<Cursor> for Kind {
	fn from(cursor: Cursor) -> Self {
		cursor.token().kind()
	}
}

impl PartialEq<Kind> for Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.1 == *other
	}
}

impl PartialEq<CommentStyle> for Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for KindSet {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<KindSet> for Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for QuoteStyle {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<QuoteStyle> for Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<char> for Cursor {
	fn eq(&self, other: &char) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for DimensionUnit {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<DimensionUnit> for Cursor {
	fn eq(&self, other: &DimensionUnit) -> bool {
		self.1 == *other
	}
}

impl PartialEq<CommentStyle> for &Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<Kind> for &Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.1 == *other
	}
}

impl PartialEq<KindSet> for &Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.1 == *other
	}
}

impl PartialEq<QuoteStyle> for &Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<char> for &Cursor {
	fn eq(&self, other: &char) -> bool {
		self.1 == *other
	}
}

impl PartialEq<DimensionUnit> for &Cursor {
	fn eq(&self, other: &DimensionUnit) -> bool {
		self.1 == *other
	}
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for Cursor {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		use serde::ser::SerializeStruct;
		let mut state = serializer.serialize_struct("Cursor", 3)?;
		state.serialize_field("kind", self.token().kind().as_str())?;
		state.serialize_field("offset", &self.offset())?;
		state.serialize_field("len", &self.token().len())?;
		if self.token().kind_bits() == Kind::Dimension as u8 {
			state.serialize_field("unit", &self.token().dimension_unit())?;
		}
		state.end()
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Cursor>(), 12);
}

#[test]
fn eq_ignore_ascii_case() {
	let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, false, 3));
	assert!(c.eq_ignore_ascii_case("foo", "foo"));
	assert!(!c.eq_ignore_ascii_case("foo", "bar"));
	assert!(!c.eq_ignore_ascii_case("fo ", "foo"));
	assert!(!c.eq_ignore_ascii_case("foo", "fooo"));
	assert!(!c.eq_ignore_ascii_case("foo", "ғоо"));

	let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 3));
	assert!(c.eq_ignore_ascii_case("FoO", "foo"));
	assert!(c.eq_ignore_ascii_case("FOO", "foo"));
	assert!(!c.eq_ignore_ascii_case("foo", "bar"));
	assert!(!c.eq_ignore_ascii_case("fo ", "foo"));
	assert!(!c.eq_ignore_ascii_case("foo", "fooo"));
	assert!(!c.eq_ignore_ascii_case("foo", "ғоо"));

	let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, false, 3));
	assert!(c.eq_ignore_ascii_case("foobar", "bar"));

	let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 3));
	assert!(c.eq_ignore_ascii_case("foobar", "bar"));

	let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 5));
	assert!(c.eq_ignore_ascii_case("foob\\61r", "bar"));

	let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 7));
	assert!(c.eq_ignore_ascii_case("foob\\61\\72", "bar"));
}
