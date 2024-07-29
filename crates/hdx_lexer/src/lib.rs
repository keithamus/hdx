use bumpalo::collections::String;
use hdx_atom::Atom;
mod constants;
mod private;
mod string_builder;
mod token;

use std::{collections::VecDeque, str::Chars};

use bitmask_enum::bitmask;
use bumpalo::Bump;
use constants::SURROGATE_RANGE;
use hdx_syntax::{identifier::is_ident, is_escape_sequence, is_whitespace, EOF, REPLACEMENT};
pub use token::{Kind, PairWise, QuoteStyle, Token};

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Include {
	Whitespace = 0b0001,
	Comments = 0b0010,
}

pub struct Lexer<'a> {
	allocator: &'a Bump,
	source: &'a str,
	chars: Chars<'a>,
	lookahead: VecDeque<Token>,
	pub include: Include,
}

impl<'a> Clone for Lexer<'a> {
	fn clone(&self) -> Self {
		self.clone_with(self.include)
	}
}

impl<'a> Lexer<'a> {
	#[inline]
	pub fn new(allocator: &'a Bump, source: &'a str, include: Include) -> Self {
		Self { allocator, source, chars: source.chars(), lookahead: VecDeque::with_capacity(4), include }
	}

	pub fn clone_with(&self, include: Include) -> Self {
		Self {
			allocator: self.allocator,
			source: self.source,
			chars: self.chars.clone(),
			lookahead: VecDeque::with_capacity(4),
			include,
		}
	}

	/// Remaining string from `Chars`
	fn remaining(&self) -> &'a str {
		self.chars.as_str()
	}

	/// Should only be used in severe edge cases, for legacy parse modes
	pub fn legacy_peek_next_char(&self, n: usize) -> Option<char> {
		self.chars.clone().nth(n)
	}

	/// Current position in file
	#[inline]
	pub fn pos(&self) -> u32 {
		(self.source.len() - self.remaining().len()) as u32
	}

	/// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
	pub fn rewind(&mut self, token: Token) {
		self.lookahead.clear();
		self.chars = self.source[token.offset as usize..].chars()
	}

	#[inline]
	pub fn advance(&mut self) -> Token {
		self.lookahead.clear();
		let token = self.read_next_token();
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
	pub fn parse_raw_str(&self, tok: Token) -> &'a str {
		let start = tok.offset as usize;
		&self.source[start..start + tok.len() as usize]
	}

	#[inline]
	pub fn parse_atom(&self, tok: Token) -> Atom {
		Atom::from(self.parse_str(tok))
	}

	#[inline]
	pub fn parse_atom_lower(&self, tok: Token) -> Atom {
		let atom = self.parse_atom(tok);
		if !tok.is_lower_case() {
			return atom.to_ascii_lowercase();
		}
		atom
	}

	#[inline]
	pub fn parse_number(&self, tok: Token) -> f32 {
		match self.source[tok.offset as usize..(tok.offset + tok.numeric_len()) as usize].parse::<f32>() {
			Ok(value) => value,
			Err(_err) => std::f32::NAN,
		}
	}

	fn parse_url_str(&self, tok: Token) -> &'a str {
		debug_assert!(tok.kind() == Kind::Url);
		// Url is special because we need to factor in that the function identifier itself can be escaped;
		let mut off = if tok.contains_escape_chars() {
			let mut chars = self.source[tok.offset as usize..].chars().peekable();
			let mut i = 0;
			loop {
				let c = chars.next().unwrap_or(EOF);
				if c == '(' {
					i += 1;
					break;
				} else if is_escape_sequence(c, *chars.peek().unwrap_or(&EOF)) {
					let (_, n) = self.parse_escape_sequence(tok.offset as usize + i);
					i += n as usize;
				} else {
					i += 1;
				}
			}
			i
		} else {
			4
		};
		if tok.url_has_leading_space() {
			// Url is also special because we need to remove leading whitespace...
			let mut chars = self.source[tok.offset as usize + off..].chars();
			while is_whitespace(chars.next().unwrap_or(EOF)) {
				off += 1;
			}
		}
		let start = tok.offset as usize + off;
		if tok.can_escape() && !tok.contains_escape_chars() {
			let end = (tok.offset + tok.len()) as usize - (tok.url_has_closing_paren() as usize);
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
							Some(String::from_str_in("", self.allocator))
						} else {
							Some(String::from_str_in(&self.source[start..(start + i)], self.allocator))
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

	pub fn parse_str(&self, tok: Token) -> &'a str {
		let kind = tok.kind();
		if kind == Kind::Url {
			return self.parse_url_str(tok);
		}
		let start = tok.offset as usize
			+ match kind {
				Kind::AtKeyword | Kind::Hash | Kind::String => 1,
				Kind::Dimension => tok.numeric_len() as usize,
				_ => 0,
			};
		if tok.can_escape() && !tok.contains_escape_chars() {
			let end = (tok.offset + tok.len()) as usize
				- match kind {
					Kind::Function => 1,
					Kind::String if tok.string_has_closing_quote() => 1,
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
						Some(String::from_str_in("", self.allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], self.allocator))
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
