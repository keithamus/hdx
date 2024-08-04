use bumpalo::Bump;
use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii, is_ident_ascii_lower, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
	is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace,
	url::{is_non_printable, is_url_ident},
	EOF,
};

use crate::{constants::SINGLE_CHAR_TOKENS, token::Token, Kind, Lexer};

impl<'a> Lexer<'a> {
	#[inline]
	fn nth_char(&self, n: usize) -> char {
		self.chars.clone().nth(n).unwrap_or(EOF)
	}

	#[must_use]
	pub(crate) fn read_next_token(&mut self) -> (Token, u32) {
		let offset = self.offset();
		if self.source.len() as u32 == offset {
			return (Token::new(Kind::Eof, 0, offset, 0), 0);
		}
		let c = self.nth_char(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let kind = SINGLE_CHAR_TOKENS[size];
			if kind != Kind::Eof {
				self.chars.next();
				return (Token::new(kind, 0, offset, c as u32), 1);
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return self.consume_ident_like_token();
			}
		}
		match c {
			'\0' => (Token::new(Kind::Eof, 0, offset, 0), 0),
			// Whitespace Range
			c if is_whitespace(c) => {
				let len = self.consume_whitespace();
				(Token::new(Kind::Whitespace, 0, offset, len), len)
			}
			// Quote Range
			c if is_quote(c) => self.consume_string_token(),
			// Digit Range
			c if c.is_ascii_digit() => self.consume_numeric_token(),
			// Sign Range
			'-' => {
				if self.nth_char(1) == '-' && self.nth_char(2) == '>' {
					self.chars.next();
					self.chars.next();
					self.chars.next();
					return (Token::new(Kind::CdcOrCdo, 1, offset, 3), 3);
				}
				if is_ident_start_sequence(c, self.nth_char(1), self.nth_char(2)) {
					return self.consume_ident_like_token();
				}
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				self.chars.next();
				(Token::new(Kind::Delim, 0, offset, '-' as u32), 1)
			}
			// Dot or Plus
			'.' | '+' => {
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				self.chars.next();
				(Token::new(Kind::Delim, 0, offset, c as u32), 1)
			}
			// Less Than
			'<' => {
				self.chars.next();
				if self.nth_char(0) == '!' && self.nth_char(0) == '-' && self.nth_char(0) == '-' {
					self.chars.next();
					self.chars.next();
					self.chars.next();
					return (Token::new(Kind::CdcOrCdo, 0, offset, 4), 4);
				}
				(Token::new(Kind::Delim, 0, offset, '<' as u32), 1)
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(self.nth_char(1)) || is_escape_sequence(self.nth_char(1), self.nth_char(2)) {
					self.consume_hash_token()
				} else {
					self.chars.next();
					(Token::new(Kind::Delim, 0, offset, '#' as u32), 1)
				}
			}
			// Commercial At
			'@' => {
				self.chars.next();
				if is_ident_start_sequence(self.nth_char(0), self.nth_char(1), self.nth_char(2)) {
					let (flags, len) = self.consume_ident_sequence();
					return (Token::new(Kind::AtKeyword, flags, offset, len + 1), len + 1);
				}
				(Token::new(Kind::Delim, 0, offset, '@' as u32), 1)
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, self.nth_char(1)) {
					return self.consume_ident_like_token();
				}
				self.chars.next();
				(Token::new(Kind::Delim, 0, offset, '\\' as u32), 1)
			}
			// Solidus
			'/' => match self.nth_char(1) {
				'*' => {
					self.chars.next();
					self.chars.next();
					let mut len = 2;
					while let Some(c) = self.chars.next() {
						len += 1;
						if c == '*' && self.nth_char(0) == '/' {
							self.chars.next();
							len += 1;
							break;
						}
					}
					(Token::new(Kind::Comment, 0, offset, len), len)
				}
				_ => {
					self.chars.next();
					(Token::new(Kind::Delim, 0, offset, '/' as u32), 1)
				}
			},
			c if is_ident_start(c) => self.consume_ident_like_token(),
			c => {
				self.chars.next();
				(Token::new(Kind::Delim, 0, offset, c as u32), 1)
			}
		}
	}

	#[must_use]
	fn consume_whitespace(&mut self) -> u32 {
		let mut i = 0;
		while is_whitespace(self.nth_char(0)) {
			self.chars.next();
			i += 1;
		}
		i
	}

	#[must_use]
	fn consume_ident_sequence(&mut self) -> (u8, u32) {
		let mut flags = 0b000;
		let mut len = 0;
		loop {
			let c = self.nth_char(0);
			if is_ident_ascii_lower(c) {
				self.chars.next();
				len += 1;
			} else if is_ident(c) {
				self.chars.next();
				len += c.len_utf8() as u32;
				flags |= 0b001;
			} else if is_escape_sequence(c, self.nth_char(1)) {
				self.chars.next();
				len += 1;
				flags |= 0b100;
				len += self.consume_escape_sequence();
			} else {
				break;
			}
		}
		(flags, len)
	}

	#[must_use]
	fn consume_escape_sequence(&mut self) -> u32 {
		let mut len = 1;
		if self.chars.next().unwrap_or(EOF).is_ascii_hexdigit() {
			let mut i = 0;
			let mut chars = self.chars.clone().peekable();
			while chars.peek().unwrap_or(&EOF).is_ascii_hexdigit() {
				chars.next();
				self.chars.next();
				len += 1;
				i += 1;
				if i > 4 {
					break;
				}
			}
			if is_whitespace(*chars.peek().unwrap_or(&EOF)) {
				self.chars.next();
				len += 1;
			}
		}
		len
	}

	#[must_use]
	fn consume_url_sequence(&mut self, offset: u32, mut len: u32, ident_flags: u8) -> (Token, u32) {
		let mut flags = ident_flags & 0b100;
		let whitespace_count = self.consume_whitespace();
		if whitespace_count > 0 {
			len += whitespace_count;
			flags |= 0b010;
		}
		loop {
			let c = self.chars.next().unwrap_or(EOF);
			len += 1;
			match c {
				')' => {
					flags |= 0b001;
					break;
				}
				EOF => {
					len -= 1;
					break;
				}
				_ if is_whitespace(c) => {
					len += self.consume_whitespace() + 1;
					// Consider trailing whitespace as escape to allow the string
					// parser to consume characters one-by-one
					flags |= 0b100;
					match self.chars.next().unwrap_or(EOF) {
						')' => {
							flags |= 0b001;
							break;
						}
						EOF => {
							break;
						}
						_ => {
							return self.consume_remnants_of_bad_url(offset, len);
						}
					};
				}
				'\'' | '"' | '(' => {
					return self.consume_remnants_of_bad_url(offset, len);
				}
				_ if is_non_printable(c) => {
					return self.consume_remnants_of_bad_url(offset, len);
				}
				'\\' => {
					if is_escape_sequence(c, self.nth_char(0)) {
						len += self.consume_escape_sequence();
						flags |= 0b100;
					} else {
						self.chars.next();
						len += 1;
						return self.consume_remnants_of_bad_url(offset, len);
					}
				}
				_ => {}
			}
		}
		(Token::new(Kind::Url, flags, offset, len), len)
	}

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, offset: u32, mut len: u32) -> (Token, u32) {
		loop {
			len += 1;
			match self.chars.next().unwrap_or(EOF) {
				')' => {
					break;
				}
				EOF => {
					break;
				}
				c @ '\\' => {
					if is_escape_sequence(c, self.nth_char(0)) {
						self.chars.next();
						len += self.consume_escape_sequence() + 1;
					}
				}
				_ => {}
			}
		}
		(Token::new(Kind::BadUrl, 0, offset, len), len)
	}

	#[must_use]
	fn consume_numeric_token(&mut self) -> (Token, u32) {
		let offset = self.offset();
		let c = self.chars.next().unwrap();
		let mut len = 1;
		let mut flags = 0b000;
		if is_sign(c) {
			flags |= 0b010;
		}
		if c == '.' {
			flags |= 0b001;
		}
		len += self.consume_decimal_digits();
		if flags & 0b001 == 0 && self.nth_char(0) == '.' && self.nth_char(1).is_ascii_digit() {
			self.chars.next();
			len += 1;
			len += self.consume_decimal_digits();
			flags |= 0b001;
		}
		if matches!(self.nth_char(0), 'e' | 'E')
			&& (self.nth_char(1).is_ascii_digit()
				|| (matches!(self.nth_char(1), '-' | '+') && self.nth_char(2).is_ascii_digit()))
		{
			self.chars.next();
			len += 1;
			if matches!(self.nth_char(0), '-' | '+') {
				self.chars.next();
				len += 1;
			}
			len += self.consume_decimal_digits();
			flags |= 0b001;
		}
		match self.nth_char(0) {
			'%' => {
				self.chars.next();
				(Token::new_dimension(flags, offset, len, 1), len + 1)
			}
			c if is_ident_start_sequence(c, self.nth_char(1), self.nth_char(2)) => {
				let (_, unit_len) = self.consume_ident_sequence();
				(Token::new_dimension(flags, offset, len, unit_len), len + unit_len)
			}
			_ => (Token::new(Kind::Number, flags, offset, len), len),
		}
	}

	#[must_use]
	fn consume_hash_token(&mut self) -> (Token, u32) {
		let offset = self.offset();
		self.chars.next();
		let hash_flags = if is_ident(self.nth_char(0)) { 0b010 } else { 0b000 };
		let (flags, len) = self.consume_ident_sequence();
		(Token::new(Kind::Hash, (flags & 0b101) | hash_flags, offset, len + 1), len + 1)
	}

	#[must_use]
	fn consume_decimal_digits(&mut self) -> u32 {
		let mut len = 0;
		while self.nth_char(0).is_ascii_digit() {
			self.chars.next();
			len += 1;
		}
		len
	}

	#[must_use]
	fn consume_ident_like_token(&mut self) -> (Token, u32) {
		let offset = self.offset();
		let (flags, mut len) = self.consume_ident_sequence();
		if self.nth_char(0) == '(' {
			self.chars.next();
			len += 1;
			let token = Token::new(Kind::Function, flags, offset, len);
			// TODO: avoid allocations here... it shouldn't be necessary
			let allocator = Bump::new();
			if is_url_ident(self.parse_str(token, &allocator)) {
				let mut chars = self.chars.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					return self.consume_url_sequence(offset, len, flags);
				}
			}
			drop(allocator);
			return (token, len);
		}
		(Token::new(Kind::Ident, flags, offset, len), len)
	}

	#[must_use]
	fn consume_string_token(&mut self) -> (Token, u32) {
		let offset = self.offset();
		let delimiter = self.chars.next().unwrap();
		let mut len = 1;
		let mut flags = (delimiter == '"') as u8;
		loop {
			match self.nth_char(0) {
				c if is_newline(c) => {
					return (Token::new(Kind::BadString, flags, offset, len), len);
				}
				EOF => {
					return (Token::new(Kind::String, flags, offset, len), len);
				}
				c @ ('"' | '\'') => {
					self.chars.next();
					len += 1;
					if c == delimiter {
						flags |= 0b010;
						return (Token::new(Kind::String, flags, offset, len), len);
					}
				}
				c @ '\\' => {
					self.chars.next();
					len += 1;
					match self.nth_char(0) {
						EOF => {
							return (Token::new(Kind::String, flags, offset, len), len);
						}
						p if is_newline(p) => {
							self.chars.next();
							len += 1;
						}
						p if is_escape_sequence(c, p) => {
							len += self.consume_escape_sequence();
							flags |= 0b100;
						}
						_ => {
							return (Token::new(Kind::BadString, flags, offset, len), len);
						}
					}
				}
				c => {
					self.chars.next();
					len += c.len_utf8() as u32;
				}
			}
		}
	}

	fn is_number_start(&mut self) -> bool {
		self.nth_char(0).is_ascii_digit()
			|| (is_sign(self.nth_char(0))
				&& (self.nth_char(1).is_ascii_digit() || self.nth_char(1) == '.' && self.nth_char(2).is_ascii_digit()))
			|| (self.nth_char(0) == '.' && self.nth_char(1).is_ascii_digit())
	}
}
