use bumpalo::collections::String;
use hdx_atom::Atom;
mod constants;
mod private;
mod span;
mod token;

use std::str::Chars;

use bitmask_enum::bitmask;
use bumpalo::Bump;
use constants::SURROGATE_RANGE;
use hdx_syntax::{identifier::is_ident, is_escape_sequence, is_whitespace, EOF, REPLACEMENT};
pub use span::{Span, Spanned};
pub use token::{Kind, PairWise, QuoteStyle, Token};

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Include {
	Whitespace = 0b0001,
	Comments = 0b0010,
}

pub struct LexerCheckpoint(pub(crate) u32);

pub struct Lexer<'a> {
	source: &'a str,
	chars: Chars<'a>,
	offset: u32,
	pub include: Include,
}

impl<'a> Clone for Lexer<'a> {
	fn clone(&self) -> Self {
		self.clone_with(self.include)
	}
}

impl<'a> Lexer<'a> {
	#[inline]
	pub fn new(source: &'a str, include: Include) -> Self {
		Self { source, chars: source.chars(), offset: 0, include }
	}

	pub fn clone_with(&self, include: Include) -> Self {
		Self { source: self.source, chars: self.chars.clone(), offset: self.offset, include }
	}

	/// Should only be used in severe edge cases, for legacy parse modes
	pub fn legacy_peek_next_char(&self, n: usize) -> Option<char> {
		self.chars.clone().nth(n)
	}

	/// Is the lexer at the end of the source
	pub fn at_end(&self) -> bool {
		self.offset == self.source.len() as u32
	}

	/// Current position in file
	#[inline(always)]
	pub fn offset(&self) -> u32 {
		self.offset
	}

	#[inline(always)]
	pub fn checkpoint(&self) -> LexerCheckpoint {
		LexerCheckpoint(self.offset)
	}

	/// Rewinds the lexer back to the given checkpoint
	pub fn rewind(&mut self, checkpoint: LexerCheckpoint) {
		debug_assert!(checkpoint.0 <= self.offset);
		// TODO: can this be optimised?
		self.chars = self.source[checkpoint.0 as usize..].chars();
		self.offset = checkpoint.0;
	}

	/// Advances the lexer to the end of the given token
	pub fn hop(&mut self, token: Token) {
		let new_offset = token.offset + token.len();
		debug_assert!(new_offset >= self.offset);
		// TODO: can this be optimised?
		self.chars = self.source[new_offset as usize..].chars();
		self.offset = new_offset;
	}

	/// Moves the lexer one token forward, returning that token
	pub fn advance(&mut self) -> Token {
		let (token, len) = self.read_next_token();
		self.offset += len;
		if token.should_skip(self.include) {
			return self.advance();
		}
		token
	}

	fn parse_escape_sequence(&self, start: usize) -> (char, u8) {
		let mut chars = self.source[start..].chars();
		let mut c = chars.next().unwrap_or(REPLACEMENT);
		if !c.is_ascii_hexdigit() {
			return (c, 1);
		}
		let mut value = 0;
		let mut i = 0;
		while let Some(v) = c.to_digit(16) {
			value = (value << 4) | v;
			i += 1;
			c = chars.next().unwrap_or(REPLACEMENT);
			if i > 5 {
				break;
			}
		}
		if is_whitespace(c) {
			i += 1;
		}
		if value == 0 || SURROGATE_RANGE.contains(&value) {
			return (REPLACEMENT, i);
		}
		(char::from_u32(value).unwrap_or(REPLACEMENT), i)
	}

	#[inline]
	pub fn parse_raw_str(&self, token: Token) -> &'a str {
		let start = token.offset as usize;
		&self.source[start..start + token.len() as usize]
	}

	#[inline]
	pub fn parse_atom(&self, token: Token, allocator: &'a Bump) -> Atom {
		Atom::from(self.parse_str(token, allocator))
	}

	#[inline]
	pub fn parse_atom_lower(&self, token: Token, allocator: &'a Bump) -> Atom {
		let atom = self.parse_atom(token, allocator);
		if !token.is_lower_case() {
			return atom.to_ascii_lowercase();
		}
		atom
	}

	#[inline]
	pub fn parse_number(&self, token: Token) -> f32 {
		match self.source[token.offset as usize..(token.offset + token.numeric_len()) as usize].parse::<f32>() {
			Ok(value) => value,
			Err(_err) => f32::NAN,
		}
	}

	fn parse_url_str(&self, token: Token, allocator: &'a Bump) -> &'a str {
		debug_assert!(token.kind() == Kind::Url);
		// Url is special because we need to factor in that the function identifier itself can be escaped;
		let mut off = if token.contains_escape_chars() {
			let mut chars = self.source[token.offset as usize..].chars().peekable();
			let mut i = 0;
			loop {
				let c = chars.next().unwrap_or(EOF);
				if c == '(' {
					i += 1;
					break;
				} else if is_escape_sequence(c, *chars.peek().unwrap_or(&EOF)) {
					let (_, n) = self.parse_escape_sequence(token.offset as usize + i);
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
			let mut chars = self.source[token.offset as usize + off..].chars();
			while is_whitespace(chars.next().unwrap_or(EOF)) {
				off += 1;
			}
		}
		let start = token.offset as usize + off;
		if token.can_escape() && !token.contains_escape_chars() {
			let end = (token.offset + token.len()) as usize - (token.url_has_closing_paren() as usize);
			return &self.source[start..end];
		}
		let mut chars = self.source[start..].chars().peekable();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		loop {
			match chars.next().unwrap_or(EOF) {
				c if c == ')' || c == EOF || is_whitespace(c) => {
					if str.is_some() {
						return str.take().unwrap().into_bump_str();
					} else {
						return &self.source[start..start + i];
					}
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
					str.as_mut().unwrap().push(ch);
					for _ in 0..n {
						chars.next();
					}
				}
				c => {
					if let Some(text) = &mut str {
						text.push(c);
					}
					i += 1;
				}
			}
		}
	}

	pub fn parse_str(&self, token: Token, allocator: &'a Bump) -> &'a str {
		let kind = token.kind();
		if kind == Kind::Url {
			return self.parse_url_str(token, allocator);
		}
		let start = token.offset as usize
			+ match kind {
				Kind::AtKeyword | Kind::Hash | Kind::String => 1,
				Kind::Dimension => token.numeric_len() as usize,
				Kind::Comment => 2,
				_ => 0,
			};
		if !token.contains_escape_chars() {
			let end = (token.offset + token.len()) as usize
				- match kind {
					Kind::Function => 1,
					Kind::String if token.string_has_closing_quote() => 1,
					Kind::Comment => 2,
					_ => 0,
				};
			return &self.source[start..end];
		}
		let mut chars = self.source[start..].chars().peekable();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		loop {
			let c = chars.next().unwrap_or(EOF);
			if is_ident(c) {
				if let Some(text) = &mut str {
					text.push(c);
				}
				i += 1;
			} else if c == '\\' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::from_str_in("", allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], allocator))
					}
				}
				i += 1;
				let (ch, n) = self.parse_escape_sequence(start + i);
				str.as_mut().unwrap().push(ch);
				for _ in 0..n {
					chars.next();
				}
			} else if str.is_some() {
				return str.take().unwrap().into_bump_str();
			} else {
				return &self.source[start..start + i];
			}
		}
	}
}
