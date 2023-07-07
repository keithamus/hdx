use hdx_atom::Atom;
use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
	is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace,
	url::{is_non_printable, is_url_ident},
	EOF, REPLACEMENT,
};

use crate::{
	constants::{SINGLE_CHAR_TOKENS, SURROGATE_RANGE},
	kind::Kind,
	string_builder::AutoCow,
	token::{Token, TokenValue},
	Lexer,
};

impl<'a> Lexer<'a> {
	pub(crate) fn finish_next(&mut self, kind: Kind) -> Token {
		self.current.token.kind = kind;
		self.current.token.span.end = self.offset();
		debug_assert!(self.current.token.span.start <= self.current.token.span.end);
		std::mem::take(&mut self.current.token)
	}

	#[inline]
	fn offset(&self) -> u32 {
		(self.source.len() - self.current.chars.as_str().len()) as u32
	}

	#[inline]
	fn peek(&self) -> char {
		self.nth(0)
	}

	#[inline]
	fn nth(&self, n: usize) -> char {
		self.current.chars.clone().nth(n).unwrap_or(EOF)
	}

	fn set_kind_and_value(&mut self, kind: Kind, s: &'a str) -> Kind {
		self.current.token.kind = kind;
		self.current.token.value = match kind {
			Kind::BadUrl | Kind::BadString => TokenValue::None,
			Kind::Url | Kind::String | Kind::Ident => TokenValue::String(Atom::from(s)),
			Kind::Function => TokenValue::String(Atom::from(s)),
			Kind::AtKeyword => TokenValue::String(Atom::from(s)),
			Kind::Hash => match s {
				_ if s.starts_with(is_ident_start) => TokenValue::String(Atom::from(s)),
				_ => TokenValue::Unrestricted(Atom::from(s)),
			},
			Kind::Number | Kind::Percentage => {
				let signed = self.current.token.value.is_signed();
				let int = self.current.token.value.is_int();
				TokenValue::Number { signed, int, value: self.parse_number(s) }
			}
			Kind::Dimension => {
				let signed = self.current.token.value.is_signed();
				let int = self.current.token.value.is_int();
				TokenValue::Dimension {
					signed,
					int,
					value: self.parse_number(s),
					unit: Atom::from(""),
				}
			}
			_ => unreachable!(),
		};
		kind
	}

	fn set_dimension_unit(&mut self, s: &'a str) {
		let signed = self.current.token.value.is_signed();
		let int = self.current.token.value.is_int();
		let value = self.current.token.value.as_f32().unwrap();
		self.current.token.value = TokenValue::Dimension { signed, int, value, unit: Atom::from(s) }
	}

	pub(crate) fn read_next_token(&mut self) -> Kind {
		self.current.token.span.start = self.offset();

		let offset = self.offset();
		self.current.token.span.start = offset;
		let builder = AutoCow::new(self);
		if let Some(c) = self.current.chars.next() {
			let kind = self.match_char(c, builder);
			if kind == Kind::Dimension {
				let mut builder = AutoCow::new(self);
				self.consume_ident_sequence(&mut builder);
				self.set_dimension_unit(builder.finish(self));
			}
			kind
		} else {
			Kind::Eof
		}
	}

	fn match_char(&mut self, c: char, mut builder: AutoCow<'a>) -> Kind {
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let kind = SINGLE_CHAR_TOKENS[size];
			if kind == Kind::Delim {
				return self.consume_delim(c);
			} else if kind != Kind::Undetermined {
				return kind;
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				builder.push_matching(c);
				let kind = self.consume_ident_like_token(&mut builder);
				return self.set_kind_and_value(kind, builder.finish(self));
			}
		}
		match c {
			// Whitespace Range
			c if is_whitespace(c) => {
				self.consume_whitespace();
				Kind::Whitespace
			}
			// Quote Range
			c if is_quote(c) => self.consume_string_token(c),
			// Digit Range
			c if c.is_ascii_digit() => {
				builder.push_matching(c);
				let kind = self.consume_numeric_token(c, &mut builder);
				return self.set_kind_and_value(kind, builder.finish(self));
			}
			// Sign Range
			'-' => {
				if self.peek() == '-' && self.nth(1) == '>' {
					self.current.chars.next();
					self.current.chars.next();
					return Kind::Cdc;
				}
				if is_ident_start_sequence(c, self.peek(), self.nth(1)) {
					builder.push_matching(c);
					let kind = self.consume_ident_like_token(&mut builder);
					return self.set_kind_and_value(kind, builder.finish(self));
				}
				if self.is_number_start(c) {
					builder.push_matching(c);
					let kind = self.consume_numeric_token(c, &mut builder);
					return self.set_kind_and_value(kind, builder.finish(self));
				}
				self.consume_delim(c)
			}
			// Dot or Plus
			'.' | '+' => {
				if self.is_number_start(c) {
					builder.push_matching(c);
					let kind = self.consume_numeric_token(c, &mut builder);
					return self.set_kind_and_value(kind, builder.finish(self));
				}
				self.consume_delim(c)
			}
			// Less Than
			'<' => {
				if self.peek() == '!' && self.nth(1) == '-' && self.nth(2) == '-' {
					self.current.chars.next();
					self.current.chars.next();
					self.current.chars.next();
					return Kind::Cdo;
				}
				self.consume_delim(c)
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(self.peek()) || is_escape_sequence(self.peek(), self.nth(1)) {
					let mut builder = AutoCow::new(self);
					self.consume_ident_sequence(&mut builder);
					return self.set_kind_and_value(Kind::Hash, builder.finish(self));
				}
				self.consume_delim(c)
			}
			// Commercial At
			'@' => {
				if is_ident_start_sequence(self.peek(), self.nth(1), self.nth(2)) {
					let mut builder = AutoCow::new(self);
					self.consume_ident_sequence(&mut builder);
					return self.set_kind_and_value(Kind::AtKeyword, builder.finish(self));
				}
				self.consume_delim(c)
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, self.peek()) {
					builder.force_allocation_without_current_ascii_char(self);
					builder.push_different(self.consume_escape_sequence());
					let kind = self.consume_ident_like_token(&mut builder);
					return self.set_kind_and_value(kind, builder.finish(self));
				}
				self.consume_delim(c)
			}
			// Solidus
			'/' => match self.peek() {
				'*' => {
					self.current.chars.next();
					self.consume_comment_token()
				}
				_ => self.consume_delim(c),
			},
			c if is_ident_start(c) => {
				builder.push_matching(c);
				let kind = self.consume_ident_like_token(&mut builder);
				self.set_kind_and_value(kind, builder.finish(self))
			}
			_ => self.consume_delim(c),
		}
	}

	fn consume_delim(&mut self, c: char) -> Kind {
		self.current.token.value = TokenValue::Char(c);
		Kind::Delim
	}

	fn consume_whitespace(&mut self) {
		loop {
			if is_whitespace(self.peek()) {
				self.current.chars.next();
			} else {
				return;
			}
		}
	}

	fn consume_ident_sequence(&mut self, builder: &mut AutoCow<'a>) {
		loop {
			let mut c = self.peek();
			if is_ident(c) {
				c = self.current.chars.next().unwrap();
				builder.push_matching(c);
			} else if is_escape_sequence(c, self.nth(1)) {
				self.current.chars.next();
				builder.force_allocation_without_current_ascii_char(self);
				builder.push_different(self.consume_escape_sequence());
			} else {
				return;
			}
		}
	}

	fn consume_escape_sequence(&mut self) -> char {
		self.current.token.escaped = true;
		if !self.peek().is_ascii_hexdigit() {
			let char = self.current.chars.next().unwrap_or(REPLACEMENT);
			return char;
		}
		if let Some(n) = self.code_point() {
			return n;
		}
		REPLACEMENT
	}

	fn consume_url_sequence(&mut self, builder: &mut AutoCow<'a>) -> Kind {
		self.consume_whitespace();
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
					if is_escape_sequence(c, self.peek()) {
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
		Kind::Url
	}

	fn consume_remnants_of_bad_url(&mut self) -> Kind {
		loop {
			match self.current.chars.next().unwrap_or(EOF) {
				')' => {
					break;
				}
				EOF => {
					break;
				}
				c @ '\\' => {
					if is_escape_sequence(c, self.peek()) {
						self.current.chars.next();
						self.consume_escape_sequence();
					}
				}
				_ => {}
			}
		}
		Kind::BadUrl
	}

	fn consume_numeric_token(&mut self, c: char, builder: &mut AutoCow<'a>) -> Kind {
		let (signed, int) = self.consume_number_sequence(c);
		match self.peek() {
			'%' => {
				self.current.chars.next();
				builder.force_allocation_without_current_ascii_char(self);
				self.current.token.value = TokenValue::Number { signed, int, value: 0.0 };
				Kind::Percentage
			}
			c if is_ident_start_sequence(c, self.nth(1), self.nth(2)) => {
				self.current.token.value =
					TokenValue::Dimension { signed, int, value: 0.0, unit: Atom::from("") };
				Kind::Dimension
			}
			_ => {
				self.current.token.value = TokenValue::Number { signed, int, value: 0.0 };
				Kind::Number
			}
		}
	}

	fn consume_number_sequence(&mut self, c: char) -> (bool, bool) {
		let signed = is_sign(c);
		let mut int = c != '.';
		self.consume_decimal_digits();
		if int && self.peek() == '.' && self.nth(1).is_ascii_digit() {
			self.current.chars.next();
			self.consume_decimal_digits();
			int = false;
		}
		if matches!(self.peek(), 'e' | 'E')
			&& (self.nth(1).is_ascii_digit()
				|| (matches!(self.nth(1), '-' | '+') && self.nth(2).is_ascii_digit()))
		{
			self.current.chars.next();
			if matches!(self.peek(), '-' | '+') {
				self.current.chars.next();
			}
			self.consume_decimal_digits();
			int = false;
		}
		(signed, int)
	}

	fn consume_decimal_digits(&mut self) {
		while self.peek().is_ascii_digit() {
			self.current.chars.next();
		}
	}

	fn consume_ident_like_token(&mut self, builder: &mut AutoCow<'a>) -> Kind {
		self.consume_ident_sequence(builder);
		if self.peek() == '(' {
			self.current.chars.next();
			let ident = builder.get_mut_string_without_current_ascii_char(self);
			if is_url_ident(ident) {
				let mut chars = self.current.chars.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					self.consume_whitespace();
					return self.consume_url_sequence(builder);
				}
			}
			return Kind::Function;
		}
		Kind::Ident
	}

	fn consume_string_token(&mut self, delimiter: char) -> Kind {
		let mut builder = AutoCow::new(self);
		loop {
			match self.peek() {
				c if is_newline(c) => {
					return Kind::BadString;
				}
				EOF => {
					return self.set_kind_and_value(Kind::String, builder.finish(self));
				}
				c @ ('"' | '\'') => {
					self.current.chars.next();
					if c == delimiter {
						return self
							.set_kind_and_value(Kind::String, builder.finish_without_push(self));
					}
					builder.push_matching(c);
				}
				'\\' => {
					let c = self.current.chars.next().unwrap();
					builder.force_allocation_without_current_ascii_char(self);
					match self.peek() {
						EOF => {
							return self.set_kind_and_value(Kind::String, builder.finish(self));
						}
						p if is_newline(p) => {
							self.current.chars.next();
						}
						p if is_escape_sequence(c, p) => {
							builder.push_different(self.consume_escape_sequence());
						}
						_ => {
							return Kind::BadString;
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

	fn consume_comment_token(&mut self) -> Kind {
		while let Some(c) = self.current.chars.next() {
			if c == '*' && self.peek() == '/' {
				self.current.chars.next();
				return Kind::Comment;
			}
		}
		Kind::Comment
	}

	fn is_number_start(&mut self, c: char) -> bool {
		c.is_ascii_digit()
			|| (is_sign(c) && self.peek().is_ascii_digit())
			|| (is_sign(c) && self.peek() == '.' && self.nth(1).is_ascii_digit())
			|| (c == '.' && self.peek().is_ascii_digit())
	}

	fn hex_digit(&mut self) -> Option<u32> {
		let value = match self.peek() {
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
		if is_whitespace(self.peek()) {
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
