use hdx_atom::{atom, Atom};
use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
	is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace,
	url::{is_non_printable, is_url_ident},
	EOF, REPLACEMENT,
};

use crate::{
	constants::{SINGLE_CHAR_TOKENS, SURROGATE_RANGE},
	string_builder::AutoCow,
	token::{NumType, Token},
	Lexer,
};

impl<'a> Lexer<'a> {
	#[inline]
	fn nth(&self, n: usize) -> char {
		self.current.chars.clone().nth(n).unwrap_or(EOF)
	}

	pub(crate) fn read_next_token(&mut self) -> Token {
		let remaining = self.current.chars.as_str();
		if remaining.is_empty() {
			return Token::Eof;
		}
		let c = self.nth(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let token = &SINGLE_CHAR_TOKENS[size];
			if token != &Token::Undetermined {
				self.current.chars.next();
				return token.clone();
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return self.consume_ident_like_token();
			}
		}
		match c {
			// Whitespace Range
			c if is_whitespace(c) => self.consume_whitespace(),
			// Quote Range
			c if is_quote(c) => self.consume_string_token(),
			// Digit Range
			c if c.is_ascii_digit() => self.consume_numeric_token(),
			// Sign Range
			'-' => {
				if self.nth(1) == '-' && self.nth(2) == '>' {
					self.current.chars.next();
					self.current.chars.next();
					self.current.chars.next();
					return Token::Cdc;
				}
				if is_ident_start_sequence(c, self.nth(1), self.nth(2)) {
					return self.consume_ident_like_token();
				}
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				Token::Delim(self.current.chars.next().unwrap())
			}
			// Dot or Plus
			'.' | '+' => {
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				Token::Delim(self.current.chars.next().unwrap())
			}
			// Less Than
			'<' => {
				if self.nth(1) == '!' && self.nth(2) == '-' && self.nth(3) == '-' {
					self.current.chars.next();
					self.current.chars.next();
					self.current.chars.next();
					self.current.chars.next();
					return Token::Cdo;
				}
				Token::Delim(self.current.chars.next().unwrap())
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(self.nth(1)) || is_escape_sequence(self.nth(1), self.nth(2)) {
					self.current.chars.next();
					self.consume_hash_token()
				} else {
					Token::Delim(self.current.chars.next().unwrap())
				}
			}
			// Commercial At
			'@' => {
				if is_ident_start_sequence(self.nth(1), self.nth(2), self.nth(3)) {
					self.current.chars.next();
					let ident = self.consume_ident_sequence();
					return Token::AtKeyword(ident);
				}
				Token::Delim(self.current.chars.next().unwrap())
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, self.nth(1)) {
					return self.consume_ident_like_token();
				}
				Token::Delim(self.current.chars.next().unwrap())
			}
			// Solidus
			'/' => match self.nth(1) {
				'*' => {
					self.current.chars.next();
					self.current.chars.next();
					self.consume_comment_token()
				}
				_ => Token::Delim(self.current.chars.next().unwrap()),
			},
			c if is_ident_start(c) => {
				return self.consume_ident_like_token();
			}
			_ => Token::Delim(self.current.chars.next().unwrap()),
		}
	}

	fn consume_whitespace(&mut self) -> Token {
		loop {
			if is_whitespace(self.nth(0)) {
				self.current.chars.next();
			} else {
				return Token::Whitespace;
			}
		}
	}

	fn consume_ident_sequence(&mut self) -> Atom {
		let mut builder = AutoCow::new(self);
		loop {
			let mut c = self.nth(0);
			if is_ident(c) {
				c = self.current.chars.next().unwrap();
				builder.push_matching(c);
			} else if is_escape_sequence(c, self.nth(1)) {
				self.current.chars.next();
				builder.force_allocation_without_current_ascii_char(self);
				builder.push_different(self.consume_escape_sequence());
			} else {
				return Atom::from(builder.finish(self));
			}
		}
	}

	fn consume_escape_sequence(&mut self) -> char {
		if !self.nth(0).is_ascii_hexdigit() {
			let char = self.current.chars.next().unwrap_or(REPLACEMENT);
			return char;
		}
		if let Some(n) = self.code_point() {
			return n;
		}
		REPLACEMENT
	}

	fn consume_url_sequence(&mut self) -> Token {
		self.consume_whitespace();
		let mut builder = AutoCow::new(self);
		builder.start = self.remaining();
		builder.value = None;
		loop {
			let c = self.current.chars.next().unwrap_or(EOF);
			match c {
				')' => {
					builder.force_allocation_without_current_ascii_char(self);
					break;
				}
				EOF => {
					break;
				}
				_ if is_whitespace(c) => {
					builder.force_allocation_without_current_ascii_char(self);
					self.consume_whitespace();
					match self.current.chars.next().unwrap_or(EOF) {
						')' => {
							break;
						}
						EOF => {
							break;
						}
						_ => {
							return self.consume_remnants_of_bad_url();
						}
					};
				}
				'\'' | '"' | '(' => {
					return self.consume_remnants_of_bad_url();
				}
				_ if is_non_printable(c) => {
					return self.consume_remnants_of_bad_url();
				}
				'\\' => {
					if is_escape_sequence(c, self.nth(0)) {
						builder.force_allocation_without_current_ascii_char(self);
						let c = self.consume_escape_sequence();
						builder.push_different(c);
					} else {
						self.current.chars.next();
						return self.consume_remnants_of_bad_url();
					}
				}
				_ => {
					builder.push_matching(c);
				}
			}
		}
		Token::Url(Atom::from(builder.finish(self)))
	}

	fn consume_remnants_of_bad_url(&mut self) -> Token {
		loop {
			match self.current.chars.next().unwrap_or(EOF) {
				')' => {
					break;
				}
				EOF => {
					break;
				}
				c @ '\\' => {
					if is_escape_sequence(c, self.nth(0)) {
						self.current.chars.next();
						self.consume_escape_sequence();
					}
				}
				_ => {}
			}
		}
		Token::BadUrl
	}

	fn consume_numeric_token(&mut self) -> Token {
		let mut builder = AutoCow::new(self);
		let c = self.current.chars.next().unwrap();
		builder.push_matching(c);
		let mut num_type = NumType::UnsignedInt;
		if is_sign(c) {
			num_type = num_type.signed();
		}
		if c == '.' {
			num_type = num_type.float();
		}
		self.consume_decimal_digits();
		if num_type.is_int() && self.nth(0) == '.' && self.nth(1).is_ascii_digit() {
			self.current.chars.next();
			self.consume_decimal_digits();
			num_type = num_type.float();
		}
		if matches!(self.nth(0), 'e' | 'E')
			&& (self.nth(1).is_ascii_digit()
				|| (matches!(self.nth(1), '-' | '+') && self.nth(2).is_ascii_digit()))
		{
			self.current.chars.next();
			if matches!(self.nth(0), '-' | '+') {
				self.current.chars.next();
			}
			self.consume_decimal_digits();
			num_type = num_type.float();
		}
		let value = self.parse_number(builder.finish(self));
		match self.nth(0) {
			'%' => {
				self.current.chars.next();
				Token::Dimension(num_type, value, atom!("%"))
			}
			c if is_ident_start_sequence(c, self.nth(1), self.nth(2)) => {
				let unit = self.consume_ident_sequence();
				Token::Dimension(num_type, value, unit)
			}
			_ => Token::Number(num_type, value),
		}
	}

	fn consume_hash_token(&mut self) -> Token {
		let ident = self.consume_ident_sequence();
		if ident.starts_with(is_ident_start) { Token::HashId(ident) } else { Token::Hash(ident) }
	}

	fn consume_decimal_digits(&mut self) {
		while self.nth(0).is_ascii_digit() {
			self.current.chars.next();
		}
	}

	fn consume_ident_like_token(&mut self) -> Token {
		let ident = self.consume_ident_sequence();
		if self.nth(0) == '(' {
			self.current.chars.next();
			if is_url_ident(&ident) {
				let mut chars = self.current.chars.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					self.consume_whitespace();
					return self.consume_url_sequence();
				}
			}
			return Token::Function(ident);
		}
		Token::Ident(ident)
	}

	fn consume_string_token(&mut self) -> Token {
		let delimiter = self.current.chars.next().unwrap();
		let mut builder = AutoCow::new(self);
		loop {
			match self.nth(0) {
				c if is_newline(c) => {
					return Token::BadString;
				}
				EOF => {
					return Token::String(Atom::from(builder.finish(self)));
				}
				c @ ('"' | '\'') => {
					self.current.chars.next();
					if c == delimiter {
						return Token::String(Atom::from(builder.finish_without_push(self)));
					}
					builder.push_matching(c);
				}
				'\\' => {
					let c = self.current.chars.next().unwrap();
					builder.force_allocation_without_current_ascii_char(self);
					match self.nth(0) {
						EOF => {
							return Token::String(Atom::from(builder.finish(self)));
						}
						p if is_newline(p) => {
							self.current.chars.next();
						}
						p if is_escape_sequence(c, p) => {
							builder.push_different(self.consume_escape_sequence());
						}
						_ => {
							return Token::BadString;
						}
					}
				}
				c => {
					self.current.chars.next();
					builder.push_matching(c);
				}
			}
		}
	}

	fn consume_comment_token(&mut self) -> Token {
		while let Some(c) = self.current.chars.next() {
			if c == '*' && self.nth(0) == '/' {
				self.current.chars.next();
				return Token::Comment;
			}
		}
		Token::Comment
	}

	fn is_number_start(&mut self) -> bool {
		self.nth(0).is_ascii_digit()
			|| (is_sign(self.nth(0))
				&& (self.nth(1).is_ascii_digit()
					|| self.nth(1) == '.' && self.nth(2).is_ascii_digit()))
			|| (self.nth(0) == '.' && self.nth(1).is_ascii_digit())
	}

	fn hex_digit(&mut self) -> Option<u32> {
		let value = match self.nth(0) {
			c if c.is_ascii_digit() => c as u32 - '0' as u32,
			c @ 'a'..='f' => 10 + (c as u32 - 'a' as u32),
			c @ 'A'..='F' => 10 + (c as u32 - 'A' as u32),
			_ => return None,
		};
		self.current.chars.next();
		Some(value)
	}

	fn code_point(&mut self) -> Option<char> {
		let mut value = self.hex_digit()?;
		let mut i = 0;
		while let Some(next) = self.hex_digit() {
			value = (value << 4) | next;
			i += 1;
			if i > 4 {
				break;
			}
		}
		if is_whitespace(self.nth(0)) {
			self.current.chars.next();
		}
		if value == 0 || SURROGATE_RANGE.contains(&value) {
			return None;
		}
		char::from_u32(value)
	}

	fn parse_number(&mut self, s: &'a str) -> f32 {
		match s.parse::<f32>() {
			Ok(value) => value,
			Err(_err) => std::f32::NAN,
		}
	}
}
