use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_lower, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
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

	pub(crate) fn read_next_token(&mut self) -> Token {
		let pos = self.pos();
		let remaining = self.chars.as_str();
		if remaining.is_empty() {
			return Token::new(Kind::Eof, 0, pos, 0);
		}
		let c = self.nth_char(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let kind = SINGLE_CHAR_TOKENS[size];
			if kind != Kind::Eof {
				self.chars.next();
				return Token::new(kind, 0, pos, c as u32);
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return self.consume_ident_like_token();
			}
		}
		match c {
			// Whitespace Range
			c if is_whitespace(c) => {
				self.consume_whitespace();
				Token::new(Kind::Whitespace, 0, pos, self.pos() - pos)
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
					return Token::new(Kind::CdcOrCdo, 1, pos, 3);
				}
				if is_ident_start_sequence(c, self.nth_char(1), self.nth_char(2)) {
					return self.consume_ident_like_token();
				}
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
			}
			// Dot or Plus
			'.' | '+' => {
				if self.is_number_start() {
					return self.consume_numeric_token();
				}
				Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
			}
			// Less Than
			'<' => {
				if self.nth_char(1) == '!' && self.nth_char(2) == '-' && self.nth_char(3) == '-' {
					self.chars.next();
					self.chars.next();
					self.chars.next();
					self.chars.next();
					return Token::new(Kind::CdcOrCdo, 0, pos, 4);
				}
				Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(self.nth_char(1)) || is_escape_sequence(self.nth_char(1), self.nth_char(2)) {
					self.consume_hash_token()
				} else {
					Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
				}
			}
			// Commercial At
			'@' => {
				if is_ident_start_sequence(self.nth_char(1), self.nth_char(2), self.nth_char(3)) {
					self.chars.next();
					let flags = self.consume_ident_sequence();
					return Token::new(Kind::AtKeyword, flags, pos, self.pos() - pos);
				}
				Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, self.nth_char(1)) {
					return self.consume_ident_like_token();
				}
				Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32)
			}
			// Solidus
			'/' => match self.nth_char(1) {
				'*' => {
					self.chars.next();
					self.chars.next();
					while let Some(c) = self.chars.next() {
						if c == '*' && self.nth_char(0) == '/' {
							self.chars.next();
							break;
						}
					}
					Token::new(Kind::Comment, 0, pos, self.pos() - pos)
				}
				_ => Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32),
			},
			c if is_ident_start(c) => self.consume_ident_like_token(),
			_ => Token::new(Kind::Delim, 0, pos, self.chars.next().unwrap() as u32),
		}
	}

	fn consume_whitespace(&mut self) -> usize {
		let mut i = 0;
		while is_whitespace(self.nth_char(0)) {
			self.chars.next();
			i += 1;
		}
		i
	}

	fn consume_ident_sequence(&mut self) -> u8 {
		let mut flags = 0b000;
		loop {
			let c = self.nth_char(0);
			if is_ident_ascii_lower(c) {
				self.chars.next();
			} else if is_ident(c) {
				self.chars.next();
				flags |= 0b001;
			} else if is_escape_sequence(c, self.nth_char(1)) {
				self.chars.next();
				flags |= 0b100;
				self.consume_escape_sequence();
			} else {
				break;
			}
		}
		flags
	}

	fn consume_escape_sequence(&mut self) {
		if self.chars.next().unwrap_or(EOF).is_ascii_hexdigit() {
			let mut i = 0;
			let mut chars = self.chars.clone().peekable();
			while chars.peek().unwrap_or(&EOF).is_ascii_hexdigit() {
				chars.next();
				self.chars.next();
				i += 1;
				if i > 4 {
					break;
				}
			}
			if is_whitespace(*chars.peek().unwrap_or(&EOF)) {
				self.chars.next();
			}
		}
	}

	fn consume_url_sequence(&mut self, pos: u32, ident_flags: u8) -> Token {
		let mut flags = ident_flags & 0b100;
		if self.consume_whitespace() > 0 {
			flags |= 0b010;
		}
		loop {
			let c = self.chars.next().unwrap_or(EOF);
			match c {
				')' => {
					flags |= 0b001;
					break;
				}
				EOF => {
					break;
				}
				_ if is_whitespace(c) => {
					self.consume_whitespace();
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
							return self.consume_remnants_of_bad_url(pos);
						}
					};
				}
				'\'' | '"' | '(' => {
					return self.consume_remnants_of_bad_url(pos);
				}
				_ if is_non_printable(c) => {
					return self.consume_remnants_of_bad_url(pos);
				}
				'\\' => {
					if is_escape_sequence(c, self.nth_char(0)) {
						self.consume_escape_sequence();
						flags |= 0b100;
					} else {
						self.chars.next();
						return self.consume_remnants_of_bad_url(pos);
					}
				}
				_ => {}
			}
		}
		Token::new(Kind::Url, flags, pos, self.pos() - pos)
	}

	fn consume_remnants_of_bad_url(&mut self, pos: u32) -> Token {
		loop {
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
						self.consume_escape_sequence();
					}
				}
				_ => {}
			}
		}
		Token::new(Kind::BadUrl, 0, pos, self.pos() - pos)
	}

	fn consume_numeric_token(&mut self) -> Token {
		let pos = self.pos();
		let c = self.chars.next().unwrap();
		let mut flags = 0b000;
		if is_sign(c) {
			flags |= 0b010;
		}
		if c == '.' {
			flags |= 0b001;
		}
		self.consume_decimal_digits();
		if flags & 0b001 == 0 && self.nth_char(0) == '.' && self.nth_char(1).is_ascii_digit() {
			self.chars.next();
			self.consume_decimal_digits();
			flags |= 0b001;
		}
		if matches!(self.nth_char(0), 'e' | 'E')
			&& (self.nth_char(1).is_ascii_digit()
				|| (matches!(self.nth_char(1), '-' | '+') && self.nth_char(2).is_ascii_digit()))
		{
			self.chars.next();
			if matches!(self.nth_char(0), '-' | '+') {
				self.chars.next();
			}
			self.consume_decimal_digits();
			flags |= 0b001;
		}
		match self.nth_char(0) {
			'%' => {
				self.chars.next();
				Token::new_dimension(flags, pos, self.pos() - pos, 1)
			}
			c if is_ident_start_sequence(c, self.nth_char(1), self.nth_char(2)) => {
				let num_pos = self.pos();
				self.consume_ident_sequence();
				Token::new_dimension(flags, pos, num_pos - pos, self.pos() - num_pos)
			}
			_ => Token::new(Kind::Number, flags, pos, self.pos() - pos),
		}
	}

	fn consume_hash_token(&mut self) -> Token {
		let pos = self.pos();
		self.chars.next();
		let hash_flags = if is_ident(self.nth_char(0)) { 0b010 } else { 0b000 };
		let flags = (self.consume_ident_sequence() & 0b101) | hash_flags;
		Token::new(Kind::Hash, flags, pos, self.pos() - pos)
	}

	fn consume_decimal_digits(&mut self) {
		while self.nth_char(0).is_ascii_digit() {
			self.chars.next();
		}
	}

	fn consume_ident_like_token(&mut self) -> Token {
		let pos = self.pos();
		let flags = self.consume_ident_sequence();
		if self.nth_char(0) == '(' {
			self.chars.next();
			let token = Token::new(Kind::Function, flags, pos, self.pos() - pos);
			if is_url_ident(self.parse_str(token)) {
				let mut chars = self.chars.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					return self.consume_url_sequence(pos, flags);
				}
			}
			return token;
		}
		Token::new(Kind::Ident, flags, pos, self.pos() - pos)
	}

	fn consume_string_token(&mut self) -> Token {
		let pos = self.pos();
		let delimiter = self.chars.next().unwrap();
		let mut flags = (delimiter == '"') as u8;
		loop {
			match self.nth_char(0) {
				c if is_newline(c) => {
					return Token::new(Kind::BadString, flags, pos, self.pos() - pos);
				}
				EOF => {
					return Token::new(Kind::String, flags, pos, self.pos() - pos);
				}
				c @ ('"' | '\'') => {
					self.chars.next();
					if c == delimiter {
						flags |= 0b010;
						return Token::new(Kind::String, flags, pos, self.pos() - pos);
					}
				}
				c @ '\\' => {
					self.chars.next();
					match self.nth_char(0) {
						EOF => {
							return Token::new(Kind::String, flags, pos, self.pos() - pos);
						}
						p if is_newline(p) => {
							self.chars.next();
						}
						p if is_escape_sequence(c, p) => {
							self.consume_escape_sequence();
							flags |= 0b100;
						}
						_ => {
							return Token::new(Kind::BadString, flags, pos, self.pos() - pos);
						}
					}
				}
				_ => {
					self.chars.next();
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
