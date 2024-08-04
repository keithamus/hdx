use bumpalo::Bump;
use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_lower, is_ident_ascii_start, is_ident_start, is_ident_start_sequence}, is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace, url::{is_non_printable, is_url_ident}, CR, EOF, LF
};

use crate::{constants::SINGLE_CHAR_TOKENS, token::Token, Kind, DimensionUnit, Lexer};

impl<'a> Lexer<'a> {
	#[inline]
	fn nth_char(&self, n: usize) -> char {
		self.chars.clone().nth(n).unwrap_or(EOF)
	}

	fn consume_newline(&mut self) -> u32 {
		if let Some(c) = self.chars.next() {
			if c == CR && self.nth_char(0) == LF {
				self.chars.next();
				return 2;
			}
		}
		1
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
				if self.nth_char(0) == '!' && self.nth_char(1) == '-' && self.nth_char(2) == '-' {
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
						len += c.len_utf8() as u32;
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
				(Token::new(Kind::Delim, 0, offset, c as u32), c.len_utf8() as u32)
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
				flags |= 0b100;
				len += self.consume_escape_sequence();
			} else {
				break;
			}
		}
		(flags, len)
	}

	#[must_use]
	fn consume_ident_sequence_finding_known_dimension(&mut self) -> (DimensionUnit, u32) {
		let mut unit = DimensionUnit::Unknown;
		let mut len = 0;
		let c = self.nth_char(0);
		if c == 'c' || c == 'C' {
			// Cap, Ch, Cm, Cqb, Cqh, Cqi, Cqmax, Cqmin, Cqw
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'a' || c == 'A' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'P' || c == 'p' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Cap
				}
			} else if c == 'h' || c == 'H' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Ch
			} else if c == 'm' || c == 'M' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Cm
			} else if c == 'q' || c == 'Q' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'b' || c == 'B' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Cqb
				} else if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Cqh
				} else if c == 'i' || c == 'I' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Cqi
				} else if c == 'm' || c == 'M' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'a' || c == 'A' {
						self.chars.next();
						len += 1;
						let c = self.nth_char(0);
						if c == 'x' || c == 'X' {
							self.chars.next();
							len += 1;
							unit = DimensionUnit::Cqmax
						}
					} else if c == 'i' || c == 'I' {
						self.chars.next();
						len += 1;
						let c = self.nth_char(0);
						if c == 'n' || c == 'N' {
							self.chars.next();
							len += 1;
							unit = DimensionUnit::Cqmin
						}
					}
				} else if c == 'w' || c == 'W' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Cqw
				}
			}
		} else if c == 'd' {
			// Deg, Dpcm, Dpi, Dppx, Dvh, Dvw
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'e' || c == 'E' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'g' || c == 'G' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Deg
				}
			} else if c == 'p' || c == 'P' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'c' || c == 'C' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'm' || c == 'M' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Dpcm
					}
				} else if c == 'i' || c == 'I' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Dpi
				} else if c == 'p' || c == 'P' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'x' || c == 'X' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Dppx
					}
				}
			} else if c == 'v' || c == 'V' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Dvh
				} else if c == 'w' || c == 'W' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Dvw
				}
			}
		} else if c == 'e' || c == 'E' {
			// Em, Ex
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'm' || c == 'M' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Em
			} else if c == 'x' || c == 'X' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Ex
			}
		} else if c == 'f' || c == 'F' {
			// Fr
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'r' || c == 'R' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Fr
			}
		} else if c == 'g' || c == 'G' {
			// Grad
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'r' || c == 'R' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'a' || c == 'A' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'd' || c == 'D' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Grad
					}
				}
			}
		} else if c == 'h' || c == 'H' {
			// Hz,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'z' || c == 'Z' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Hz
			}
		} else if c == 'i' || c == 'I' {
			// Ic, In,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'c' || c == 'C' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Ic
			} else if c == 'n' || c == 'N' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::In
			}
		} else if c == 'k' || c == 'K' {
			// KHz,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'z' || c == 'Z' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'z' || c == 'Z' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::KHz
				}
			}
		} else if c == 'l' || c == 'L' {
			// Lh, Lvh, Lvw,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'h' || c == 'H' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Lh
			} else if c == 'v' || c == 'V' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Lvh
				} else if c == 'w' || c == 'W' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Lvw
				}
			}
		} else if c == 'm' || c == 'M' {
			// Mm, Ms,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'm' || c == 'M' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Mm
			} else if c == 's' || c == 'S' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Ms
			}
		} else if c == 'p' || c == 'P' {
			// Pc, Pt, Px,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'c' || c == 'C' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Pc
			} else if c == 't' || c == 'T' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Pt
			} else if c == 'x' || c == 'X' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Px
			}
		} else if c == 'q' || c == 'Q' {
			// Q,
			self.chars.next();
			len += 1;
			unit = DimensionUnit::Q
		} else if c == 'r' || c == 'R' {
			// Rad, Rcap, Rch, Rem, Rex, Ric, Rlh,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'a' || c == 'A' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'd' || c == 'D' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Rad
				}
			} else if c == 'c' || c == 'C' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'a' || c == 'A' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'p' || c == 'P' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Rcap
					}
				} else if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Rch
				}
			} else if c == 'e' || c == 'E' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'm' || c == 'M' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Rem
				} else if c == 'x' || c == 'X' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Rex
				}
			} else if c == 'i' || c == 'I' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'c' || c == 'C' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Ric
				}
			} else if c == 'l' || c == 'L' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Rlh
				}
			}
		} else if c == 's' || c == 'S' {
			// S, Svh, Svw,
			self.chars.next();
			len += 1;
			unit = DimensionUnit::S;
			let c = self.nth_char(0);
			if c == 'v' || c == 'V' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'h' || c == 'H' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Svh
				} else if c == 'w' || c == 'W' {
					self.chars.next();
					len += 1;
					unit = DimensionUnit::Svw
				}
			}
		} else if c == 't' || c == 'T' {
			// Turn,
		} else if c == 'v' || c == 'V' {
			// Vb, Vh, Vi, Vmax, Vmin, Vw,
			self.chars.next();
			len += 1;
			let c = self.nth_char(0);
			if c == 'b' || c == 'B' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Vb
			} else if c == 'h' || c == 'H' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Vh
			} else if c == 'i' || c == 'I' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Vi
			} else if c == 'm' || c == 'M' {
				self.chars.next();
				len += 1;
				let c = self.nth_char(0);
				if c == 'a' || c == 'A' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'x' || c == 'X' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Vmax
					}
				} else if c == 'i' || c == 'I' {
					self.chars.next();
					len += 1;
					let c = self.nth_char(0);
					if c == 'n' || c == 'N' {
						self.chars.next();
						len += 1;
						unit = DimensionUnit::Vmin
					}
				}
			} else if c == 'w' || c == 'W' {
				self.chars.next();
				len += 1;
				unit = DimensionUnit::Vw
			}
		} else if c == 'x' || c == 'X' {
			// X,
			self.chars.next();
			len += 1;
			unit = DimensionUnit::X
		}
		let (_, rest_len) = self.consume_ident_sequence();
		if rest_len > 0 {
			(DimensionUnit::Unknown, len + rest_len)
		} else {
			(unit, len)
		}
	}

	#[must_use]
	fn consume_escape_sequence(&mut self) -> u32 {
		let mut len = 1;
		if let Some(c) = self.chars.next() {
			len += 1;
			if c.is_ascii_hexdigit() {
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
		}
		len
	}

	#[must_use]
	fn consume_url_sequence(&mut self, offset: u32, len: u32, ident_flags: u8) -> (Token, u32) {
		let mut len = len;
		let mut flags = ident_flags & 0b100;
		let whitespace_count = self.consume_whitespace();
		if whitespace_count > 0 {
			len += whitespace_count;
			flags |= 0b010;
		}
		loop {
			let c = self.nth_char(0);
			match c {
				')' => {
					self.chars.next();
					len += 1;
					flags |= 0b001;
					break;
				}
				EOF => {
					break;
				}
				_ if is_whitespace(c) => {
					len += self.consume_whitespace();
					// Consider trailing whitespace as escape to allow the string
					// parser to consume characters one-by-one
					flags |= 0b100;
					match self.chars.next().unwrap_or(EOF) {
						')' => {
							len += 1;
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
						self.chars.next();
						len += self.consume_escape_sequence();
						flags |= 0b100;
					} else {
						self.chars.next();
						len += 1;
						return self.consume_remnants_of_bad_url(offset, len);
					}
				}
				c => {
					self.chars.next();
					len += c.len_utf8() as u32;
				}
			}
		}
		(Token::new(Kind::Url, flags, offset, len), len)
	}

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, offset: u32, len: u32) -> (Token, u32) {
		let mut len = len;
		loop {
			match self.chars.next().unwrap_or(EOF) {
				')' => {
					len += 1;
					break;
				}
				EOF => {
					break;
				}
				c @ '\\' => {
					len += 1;
					if is_escape_sequence(c, self.nth_char(0)) {
						self.chars.next();
						len += self.consume_escape_sequence();
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
		let mut flags = 0b100;
		let mut sign = 1;
		if is_sign(c) {
			if c == '-' {
				sign = -1;
			}
			flags |= 0b010;
		}
		if c == '.' {
			flags &= 0b011;
			flags |= 0b001;
		}
		let r = self.consume_decimal_digits_returning_val(if c.is_ascii_digit() { c as u32 - 48 } else { 0 });
		let acc = r.0 as i32 * (sign);
		len += r.1;
		dbg!(self.nth_char(0), self.nth_char(1));
		if flags & 0b001 == 0 && self.nth_char(0) == '.' && self.nth_char(1).is_ascii_digit() {
			self.chars.next();
			len += 1;
			len += self.consume_decimal_digits();
			flags &= 0b011;
			dbg!("Setting 0b011");
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
			flags &= 0b011;
			flags |= 0b001;
		}
		dbg!(format!("{:#08b}", flags));
		match self.nth_char(0) {
			'%' => {
				self.chars.next();
				(Token::new_dimension(flags, offset, len, 1, acc, DimensionUnit::Percent), len + 1)
			}
			c if is_ident_start_sequence(c, self.nth_char(1), self.nth_char(2)) => {
				let (known_unit, unit_len) = self.consume_ident_sequence_finding_known_dimension();
				(Token::new_dimension(flags, offset, len, unit_len, acc, known_unit), len + unit_len)
			}
			_ => (Token::new_number(flags, offset, len, acc), len),
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
	fn consume_decimal_digits_returning_val(&mut self, acc: u32) -> (u32, u32) {
		let mut acc = acc;
		let mut len = 0;
		while self.nth_char(0).is_ascii_digit() {
			acc = (acc * 10) + (self.chars.next().unwrap() as u32 - 48);
			len += 1;
		}
		(acc, len)
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
					flags |= 0b100;
					match self.nth_char(0) {
						EOF => {
							len += 1;
							return (Token::new(Kind::String, flags, offset, len), len);
						}
						p if is_newline(p) => {
							len += self.consume_newline() + 1;
						}
						p if is_escape_sequence(c, p) => {
							len += self.consume_escape_sequence();
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
