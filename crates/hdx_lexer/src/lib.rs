use bumpalo::collections::String;
use hdx_atom::{Atom, Atomizable};
mod constants;
mod dimension_unit;
mod private;
mod span;
mod token;

use std::char::REPLACEMENT_CHARACTER;

use bitmask_enum::bitmask;
use bumpalo::Bump;
use constants::SURROGATE_RANGE;
pub use dimension_unit::DimensionUnit;
use hdx_syntax::{is_escape_sequence, is_newline, is_whitespace, EOF};
pub use span::{Span, Spanned};
pub use token::{Kind, PairWise, QuoteStyle, Token};

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Include {
	Whitespace = 0b0001,
	Comments = 0b0010,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Feature {
	TokenizeDoubleSlashesAsComments = 0b0001,
}

#[derive(Copy, Clone, PartialEq, Default, Hash)]
pub struct LexerCheckpoint(pub(crate) Token);

pub struct Lexer<'a> {
	source: &'a str,
	current_token: Token,
	pub include: Include,
	features: Feature,
}

impl<'a> Clone for Lexer<'a> {
	fn clone(&self) -> Self {
		self.clone_with(self.include)
	}
}

impl<'a> Lexer<'a> {
	#[inline]
	pub fn new(source: &'a str, include: Include) -> Self {
		Self { source, current_token: Token::new(Kind::Whitespace, 0, 0, 0), include, features: Feature::none() }
	}

	#[inline]
	pub fn new_with_features(source: &'a str, include: Include, features: Feature) -> Self {
		Self { source, current_token: Token::new(Kind::Whitespace, 0, 0, 0), include, features }
	}

	pub fn clone_with(&self, include: Include) -> Self {
		Self { source: self.source, current_token: self.current_token, include, features: self.features }
	}

	/// Is the lexer at the last token
	pub fn at_end(&self) -> bool {
		self.current_token.end_offset() == self.source.len() as u32
	}

	/// Current position in file
	#[inline(always)]
	pub fn offset(&self) -> u32 {
		self.current_token.end_offset()
	}

	#[inline(always)]
	pub fn checkpoint(&self) -> LexerCheckpoint {
		LexerCheckpoint(self.current_token)
	}

	/// Rewinds the lexer back to the given checkpoint
	pub fn rewind(&mut self, checkpoint: LexerCheckpoint) {
		debug_assert!(checkpoint.0.offset() <= self.offset());
		self.current_token = checkpoint.0
	}

	/// Advances the lexer to the end of the given token
	pub fn hop(&mut self, token: Token) {
		debug_assert!(token.offset() >= self.current_token.end_offset());
		self.current_token = token;
	}

	/// Moves the lexer one token forward, returning that token
	pub fn advance(&mut self) -> Token {
		self.current_token = self.read_next_token();
		if self.current_token.should_skip(self.include) {
			return self.advance();
		}
		self.current_token
	}

	fn parse_escape_sequence(&self, start: usize) -> (char, u8) {
		let mut chars = self.source[start..].chars();
		if let Some(c) = chars.next() {
			if !c.is_ascii_hexdigit() {
				return (c, c.len_utf8() as u8);
			}
			let mut value = 0;
			let mut i = 0;
			let mut c = c;
			while let Some(v) = c.to_digit(16) {
				value = (value << 4) | v;
				i += 1;
				c = chars.next().unwrap_or(REPLACEMENT_CHARACTER);
				if i > 5 {
					break;
				}
			}
			if is_whitespace(c) {
				i += 1;
				if c == '\r' && chars.next() == Some('\n') {
					i += 1;
				}
			}
			if value == 0 || SURROGATE_RANGE.contains(&value) {
				return (REPLACEMENT_CHARACTER, i);
			}
			(char::from_u32(value).unwrap_or(REPLACEMENT_CHARACTER), i)
		} else {
			(REPLACEMENT_CHARACTER, 0)
		}
	}

	#[inline]
	pub fn parse_raw_str(&self, token: Token) -> &'a str {
		&self.source[token.offset() as usize..token.end_offset() as usize]
	}

	#[inline]
	pub fn parse_atom(&self, token: Token, allocator: &'a Bump) -> Atom {
		Atom::from(self.parse_str(token, allocator))
	}

	#[inline]
	pub fn parse_atom_lower(&self, token: Token, allocator: &'a Bump) -> Atom {
		if token.kind_bits() == Kind::Dimension as u8 {
			let unit = token.dimension_unit();
			if unit != DimensionUnit::Unknown {
				return unit.to_atom();
			}
		}
		Atom::from(self.parse_str(token, allocator).to_ascii_lowercase())
	}

	#[inline]
	pub fn parse_number(&self, token: Token) -> f32 {
		if let Some(n) = token.stored_small_number() {
			return n;
		}
		match self.source[token.offset() as usize..(token.offset() + token.numeric_len()) as usize].parse::<f32>() {
			Ok(value) => value,
			Err(_err) => f32::NAN,
		}
	}

	fn parse_url_str(&self, token: Token, allocator: &'a Bump) -> &'a str {
		debug_assert!(token.kind() == Kind::Url);
		// Url is special because we need to factor in that the function identifier itself can be escaped;
		let mut off = if token.contains_escape_chars() {
			let mut chars = self.source[token.offset() as usize..].chars().peekable();
			let mut i = 0;
			while let Some(c) = chars.next() {
				if c == '(' {
					i += 1;
					break;
				} else if is_escape_sequence(c, *chars.peek().unwrap_or(&EOF)) {
					let (_, n) = self.parse_escape_sequence(token.offset() as usize + i);
					i += n as usize;
				} else {
					i += 1;
				}
			}
			i
		} else {
			4
		};
		if token.url_has_leading_space() {
			// Url is also special because we need to remove leading whitespace...
			let mut chars = self.source[token.offset() as usize + off..].chars();
			while is_whitespace(chars.next().unwrap_or(EOF)) {
				off += 1;
			}
		}
		let start = token.offset() as usize + off;
		let end = token.end_offset() as usize - (token.url_has_closing_paren() as usize);
		if token.can_escape() && !token.contains_escape_chars() {
			return &self.source[start..end];
		}
		let mut chars = self.source[start..end].chars();
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
							Some(String::from_str_in(&self.source[start..(start + i)], allocator))
						}
					}
					i += 1;
					let (ch, n) = self.parse_escape_sequence(start + i);
					if is_newline(c) && self.source[(start + i + (n as usize))..].starts_with('\n') {
						i += 1;
					}
					str.as_mut().unwrap().push(ch);
					i += n as usize;
					chars = self.source[(start + i)..end].chars();
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
			&self.source[start..start + i]
		}
	}

	pub fn parse_str(&self, token: Token, allocator: &'a Bump) -> &'a str {
		let kind = token.kind();
		if kind == Kind::Url {
			return self.parse_url_str(token, allocator);
		}
		if kind == Kind::Delim {
			let mut str = String::from_str_in("", allocator);
			str.push(token.char().unwrap());
			return str.into_bump_str();
		}
		let start = token.offset() as usize
			+ match kind {
				Kind::AtKeyword | Kind::Hash | Kind::String => 1,
				Kind::Dimension => token.numeric_len() as usize,
				Kind::Comment => 2,
				_ => 0,
			};
		let end = token.end_offset() as usize
			- match kind {
				Kind::Function => 1,
				Kind::String if token.string_has_closing_quote() => 1,
				Kind::Comment => 2,
				_ => 0,
			};
		if !token.contains_escape_chars() {
			return &self.source[start..end];
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		while let Some(c) = chars.next() {
			if c == '\0' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::from_str_in("", allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], allocator))
					}
				}
				str.as_mut().unwrap().push(REPLACEMENT_CHARACTER);
				i += 1;
			} else if c == '\\' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::from_str_in("", allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], allocator))
					}
				}
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-token
				if token.kind_bits() == Kind::String as u8 {
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
							chars = self.source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = self.parse_escape_sequence(start + i);
				str.as_mut().unwrap().push(if ch == '\0' { REPLACEMENT_CHARACTER } else { ch });
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
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
			&self.source[start..start + i]
		}
	}
}
