use crate::{
	span::SpanContents,
	syntax::{is_newline, ParseEscape},
	CommentStyle, DimensionUnit, Kind, KindSet, QuoteStyle, SourceOffset, Span, Token,
};
use bumpalo::{collections::String, Bump};
use std::{char::REPLACEMENT_CHARACTER, fmt};

/// Wraps [Token] with a [SourceOffset], allows it to reason about the character data of the source text.
///
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor(SourceOffset, Token);

impl Cursor {
	pub const DUMMY_SITE_NUMBER_ZERO: Cursor = Cursor(SourceOffset::DUMMY, Token::NUMBER_ZERO);

	#[inline(always)]
	pub const fn new(offset: SourceOffset, token: Token) -> Self {
		Self(offset, token)
	}

	#[inline(always)]
	pub const fn dummy(token: Token) -> Self {
		Self(SourceOffset::DUMMY, token)
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.1
	}

	#[inline(always)]
	pub const fn offset(&self) -> SourceOffset {
		self.0
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
					f.write_str(d.into())?;
				}
			},
			Kind::Comment
			| Kind::Whitespace
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
		let start = (self.offset().0 + self.token().leading_len()) as usize;
		let end = (self.end_offset().0 - self.token().trailing_len()) as usize;
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
		let start = (self.offset().0 + self.token().leading_len()) as usize;
		let end = (self.end_offset().0 - self.token().trailing_len()) as usize;
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
						Some(String::new_in(allocator))
					} else {
						Some(String::from_str_in(&source[start..(start + i)], allocator))
					}
				}
				str.as_mut().unwrap().push(REPLACEMENT_CHARACTER);
				i += 1;
			} else if c == '\\' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::new_in(allocator))
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

	#[inline]
	pub fn parse_str_lower<'a>(&self, source: &'a str, allocator: &'a Bump) -> &'a str {
		debug_assert!(self != Kind::Delim);
		if self.token().is_lower_case() {
			return self.parse_str(source, allocator);
		}
		let start = (self.offset().0 + self.token().leading_len()) as usize;
		let end = (self.end_offset().0 - self.token().trailing_len()) as usize;
		if !self.token().contains_escape_chars() && self.token().is_lower_case() {
			return &source[start..end];
		}
		let mut chars = source[start..end].chars().peekable();
		let mut i = 0;
		let mut str: String<'a> = String::new_in(allocator);
		while let Some(c) = chars.next() {
			if c == '\0' {
				str.push(REPLACEMENT_CHARACTER);
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
				str.push(if ch == '\0' { REPLACEMENT_CHARACTER } else { ch.to_ascii_lowercase() });
				i += n as usize;
				chars = source[(start + i)..end].chars().peekable();
			} else {
				str.push(c.to_ascii_lowercase());
				i += c.len_utf8();
			}
		}
		str.into_bump_str()
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
fn parse_str_lower() {
	let allocator = Bump::new();
	let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 3));
	assert_eq!(c.parse_str_lower("FoO", &allocator), "foo");
	assert_eq!(c.parse_str_lower("FOO", &allocator), "foo");
	assert_eq!(c.parse_str_lower("foo", &allocator), "foo");

	let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, false, 5));
	assert_eq!(c.parse_str_lower("'FoO'", &allocator), "foo");
	assert_eq!(c.parse_str_lower("'FOO'", &allocator), "foo");

	let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, false, false, 4));
	assert_eq!(c.parse_str_lower("'FoO", &allocator), "foo");
	assert_eq!(c.parse_str_lower("'FOO", &allocator), "foo");
	assert_eq!(c.parse_str_lower("'foo", &allocator), "foo");
	assert_eq!(c.parse_str_lower("'foo", &allocator), "foo");

	let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 4, 1, 6));
	assert_eq!(c.parse_str_lower("url(a)", &allocator), "a");
	assert_eq!(c.parse_str_lower("url(b)", &allocator), "b");

	let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 6, 1, 8));
	assert_eq!(c.parse_str_lower("\\75rl(A)", &allocator), "a");
	assert_eq!(c.parse_str_lower("u\\52l(B)", &allocator), "b");
	assert_eq!(c.parse_str_lower("ur\\6c(C)", &allocator), "c");

	let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 8, 1, 10));
	assert_eq!(c.parse_str_lower("\\75\\52l(A)", &allocator), "a");
	assert_eq!(c.parse_str_lower("u\\52\\6c(B)", &allocator), "b");
	assert_eq!(c.parse_str_lower("\\75r\\6c(C)", &allocator), "c");
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
