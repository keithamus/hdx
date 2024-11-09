use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_lower, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
	is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace,
	url::is_non_printable,
	CR, EOF, LF,
};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

use crate::{constants::SINGLE_CHAR_TOKENS, token::Token, DimensionUnit, Kind, Lexer};

trait CharsConsumer {
	fn is_last(&self) -> bool;

	fn peek_nth(&self, n: usize) -> char;

	#[must_use]
	fn consume_newline(&mut self) -> u32;

	#[must_use]
	fn consume_whitespace(&mut self) -> u32;

	#[must_use]
	fn consume_ident_sequence(&mut self) -> (u8, u32);

	#[must_use]
	fn consume_ident_sequence_finding_known_dimension(&mut self) -> (DimensionUnit, u32);

	#[must_use]
	fn consume_ident_sequence_finding_url_keyword(&mut self) -> (u8, u32, bool);

	#[must_use]
	fn consume_escape_sequence(&mut self) -> u32;

	#[must_use]
	fn consume_url_sequence(&mut self, offset: u32, len: u32, ident_flags: u8) -> Token;

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, offset: u32, len: u32) -> Token;

	#[must_use]
	fn consume_numeric_token(&mut self, offset: u32) -> Token;

	#[must_use]
	fn consume_hash_token(&mut self, offset: u32) -> Token;

	#[must_use]
	fn consume_decimal_digits_returning_val(&mut self, acc: u32) -> (u32, u32);

	#[must_use]
	fn consume_decimal_digits(&mut self) -> u32;

	#[must_use]
	fn consume_ident_like_token(&mut self, offset: u32) -> Token;

	#[must_use]
	fn consume_string_token(&mut self, offset: u32) -> Token;

	#[must_use]
	fn is_number_start(&mut self) -> bool;
}

impl<'a> CharsConsumer for Chars<'a> {
	#[inline]
	fn is_last(&self) -> bool {
		self.clone().next().is_none()
	}

	#[inline]
	fn peek_nth(&self, n: usize) -> char {
		self.clone().nth(n).unwrap_or(EOF)
	}

	#[must_use]
	fn consume_newline(&mut self) -> u32 {
		if let Some(c) = self.next() {
			if c == CR && self.peek_nth(0) == LF {
				self.next();
				return 2;
			}
		}
		1
	}

	#[must_use]
	fn consume_whitespace(&mut self) -> u32 {
		let mut i = 0;
		while is_whitespace(self.peek_nth(0)) {
			self.next();
			i += 1;
		}
		i
	}

	#[must_use]
	fn consume_ident_sequence(&mut self) -> (u8, u32) {
		let mut flags = 0b000;
		let mut len = 0;
		loop {
			let c = self.peek_nth(0);
			if is_ident_ascii_lower(c) {
				self.next();
				len += 1;
			} else if is_ident(c) {
				self.next();
				len += c.len_utf8() as u32;
				flags |= 0b001;
			} else if is_escape_sequence(c, self.peek_nth(1)) {
				self.next();
				flags |= 0b100;
				len += self.consume_escape_sequence();
			} else if c == '\0' && self.next().is_some() {
				// Set the escape flag to ensure \0s get replaced
				flags |= 0b100;
				len += 1;
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
		let c = self.peek_nth(0);
		if c == 'c' || c == 'C' {
			// Cap, Ch, Cm, Cqb, Cqh, Cqi, Cqmax, Cqmin, Cqw
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'a' || c == 'A' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'P' || c == 'p' {
					self.next();
					len += 1;
					unit = DimensionUnit::Cap
				}
			} else if c == 'h' || c == 'H' {
				self.next();
				len += 1;
				unit = DimensionUnit::Ch
			} else if c == 'm' || c == 'M' {
				self.next();
				len += 1;
				unit = DimensionUnit::Cm
			} else if c == 'q' || c == 'Q' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'b' || c == 'B' {
					self.next();
					len += 1;
					unit = DimensionUnit::Cqb
				} else if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Cqh
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					unit = DimensionUnit::Cqi
				} else if c == 'm' || c == 'M' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'a' || c == 'A' {
						self.next();
						len += 1;
						let c = self.peek_nth(0);
						if c == 'x' || c == 'X' {
							self.next();
							len += 1;
							unit = DimensionUnit::Cqmax
						}
					} else if c == 'i' || c == 'I' {
						self.next();
						len += 1;
						let c = self.peek_nth(0);
						if c == 'n' || c == 'N' {
							self.next();
							len += 1;
							unit = DimensionUnit::Cqmin
						}
					}
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Cqw
				}
			}
		} else if c == 'd' {
			// Deg, Dpcm, Dpi, Dppx, Dvh, Dvw
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'e' || c == 'E' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'g' || c == 'G' {
					self.next();
					len += 1;
					unit = DimensionUnit::Deg
				}
			} else if c == 'p' || c == 'P' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'c' || c == 'C' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'm' || c == 'M' {
						self.next();
						len += 1;
						unit = DimensionUnit::Dpcm
					}
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dpi
				} else if c == 'p' || c == 'P' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'x' || c == 'X' {
						self.next();
						len += 1;
						unit = DimensionUnit::Dppx
					}
				}
			} else if c == 'v' || c == 'V' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvh
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvw
				}
			}
		} else if c == 'e' || c == 'E' {
			// Em, Ex
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'm' || c == 'M' {
				self.next();
				len += 1;
				unit = DimensionUnit::Em
			} else if c == 'x' || c == 'X' {
				self.next();
				len += 1;
				unit = DimensionUnit::Ex
			}
		} else if c == 'f' || c == 'F' {
			// Fr
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'r' || c == 'R' {
				self.next();
				len += 1;
				unit = DimensionUnit::Fr
			}
		} else if c == 'g' || c == 'G' {
			// Grad
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'r' || c == 'R' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'a' || c == 'A' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'd' || c == 'D' {
						self.next();
						len += 1;
						unit = DimensionUnit::Grad
					}
				}
			}
		} else if c == 'h' || c == 'H' {
			// Hz,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'z' || c == 'Z' {
				self.next();
				len += 1;
				unit = DimensionUnit::Hz
			}
		} else if c == 'i' || c == 'I' {
			// Ic, In,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'c' || c == 'C' {
				self.next();
				len += 1;
				unit = DimensionUnit::Ic
			} else if c == 'n' || c == 'N' {
				self.next();
				len += 1;
				unit = DimensionUnit::In
			}
		} else if c == 'k' || c == 'K' {
			// KHz,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'z' || c == 'Z' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'z' || c == 'Z' {
					self.next();
					len += 1;
					unit = DimensionUnit::KHz
				}
			}
		} else if c == 'l' || c == 'L' {
			// Lh, Lvh, Lvw,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'h' || c == 'H' {
				self.next();
				len += 1;
				unit = DimensionUnit::Lh
			} else if c == 'v' || c == 'V' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvh
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvw
				}
			}
		} else if c == 'm' || c == 'M' {
			// Mm, Ms,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'm' || c == 'M' {
				self.next();
				len += 1;
				unit = DimensionUnit::Mm
			} else if c == 's' || c == 'S' {
				self.next();
				len += 1;
				unit = DimensionUnit::Ms
			}
		} else if c == 'p' || c == 'P' {
			// Pc, Pt, Px,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'c' || c == 'C' {
				self.next();
				len += 1;
				unit = DimensionUnit::Pc
			} else if c == 't' || c == 'T' {
				self.next();
				len += 1;
				unit = DimensionUnit::Pt
			} else if c == 'x' || c == 'X' {
				self.next();
				len += 1;
				unit = DimensionUnit::Px
			}
		} else if c == 'q' || c == 'Q' {
			// Q,
			self.next();
			len += 1;
			unit = DimensionUnit::Q
		} else if c == 'r' || c == 'R' {
			// Rad, Rcap, Rch, Rem, Rex, Ric, Rlh,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'a' || c == 'A' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'd' || c == 'D' {
					self.next();
					len += 1;
					unit = DimensionUnit::Rad
				}
			} else if c == 'c' || c == 'C' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'a' || c == 'A' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'p' || c == 'P' {
						self.next();
						len += 1;
						unit = DimensionUnit::Rcap
					}
				} else if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Rch
				}
			} else if c == 'e' || c == 'E' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'm' || c == 'M' {
					self.next();
					len += 1;
					unit = DimensionUnit::Rem
				} else if c == 'x' || c == 'X' {
					self.next();
					len += 1;
					unit = DimensionUnit::Rex
				}
			} else if c == 'i' || c == 'I' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'c' || c == 'C' {
					self.next();
					len += 1;
					unit = DimensionUnit::Ric
				}
			} else if c == 'l' || c == 'L' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Rlh
				}
			}
		} else if c == 's' || c == 'S' {
			// S, Svh, Svw,
			self.next();
			len += 1;
			unit = DimensionUnit::S;
			let c = self.peek_nth(0);
			if c == 'v' || c == 'V' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svh
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svw
				}
			}
		} else if c == 't' || c == 'T' {
			// Turn,
		} else if c == 'v' || c == 'V' {
			// Vb, Vh, Vi, Vmax, Vmin, Vw,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'b' || c == 'B' {
				self.next();
				len += 1;
				unit = DimensionUnit::Vb
			} else if c == 'h' || c == 'H' {
				self.next();
				len += 1;
				unit = DimensionUnit::Vh
			} else if c == 'i' || c == 'I' {
				self.next();
				len += 1;
				unit = DimensionUnit::Vi
			} else if c == 'm' || c == 'M' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'a' || c == 'A' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'x' || c == 'X' {
						self.next();
						len += 1;
						unit = DimensionUnit::Vmax
					}
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'n' || c == 'N' {
						self.next();
						len += 1;
						unit = DimensionUnit::Vmin
					}
				}
			} else if c == 'w' || c == 'W' {
				self.next();
				len += 1;
				unit = DimensionUnit::Vw
			}
		} else if c == 'x' || c == 'X' {
			// X,
			self.next();
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
		if let Some(c) = self.next() {
			len += 1;
			if c.is_ascii_hexdigit() {
				let mut i = 0;
				let mut chars = self.clone().peekable();
				while chars.peek().unwrap_or(&EOF).is_ascii_hexdigit() {
					chars.next();
					self.next();
					len += 1;
					i += 1;
					if i > 4 {
						break;
					}
				}
				if is_whitespace(*chars.peek().unwrap_or(&EOF)) {
					let c = self.next();
					len += 1;
					// https://drafts.csswg.org/css-syntax/#input-preprocessing
					// Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED (FF) code points,
					// or pairs of U+000D CARRIAGE RETURN (CR) followed by U+000A LINE FEED (LF) in input by
					// single U+000A LINE FEED (LF) code point.
					if c == Some('\r') && self.peek_nth(0) == '\n' {
						self.next();
						len += 1;
					}
				}
			}
		}
		len
	}

	#[must_use]
	fn consume_url_sequence(&mut self, offset: u32, len: u32, ident_flags: u8) -> Token {
		let mut len = len;
		let mut flags = ident_flags & 0b100;
		let whitespace_count = self.consume_whitespace();
		if whitespace_count > 0 {
			len += whitespace_count;
			flags |= 0b010;
		}
		loop {
			let c = self.peek_nth(0);
			match c {
				')' => {
					self.next();
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
					match self.next().unwrap_or(EOF) {
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
					if is_escape_sequence(c, self.peek_nth(0)) {
						self.next();
						len += self.consume_escape_sequence();
						flags |= 0b100;
					} else {
						len += 1;
						return self.consume_remnants_of_bad_url(offset, len);
					}
				}
				c => {
					self.next();
					len += c.len_utf8() as u32;
				}
			}
		}
		Token::new(Kind::Url, flags, offset, len)
	}

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, offset: u32, len: u32) -> Token {
		let mut len = len;
		while let Some(ch) = self.next() {
			match ch {
				')' => {
					len += 1;
					break;
				}
				'\\' => {
					len += 1;
					if is_escape_sequence(ch, self.peek_nth(0)) {
						len += self.consume_escape_sequence();
					} else if let Some(ch) = self.next() {
						len += ch.len_utf8() as u32;
					}
				}
				_ => {
					len += ch.len_utf8() as u32;
				}
			}
		}
		Token::new(Kind::BadUrl, 0, offset, len)
	}

	#[must_use]
	fn consume_numeric_token(&mut self, offset: u32) -> Token {
		let c = self.next().unwrap();
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
		if flags & 0b001 == 0 && self.peek_nth(0) == '.' && self.peek_nth(1).is_ascii_digit() {
			self.next();
			len += 1;
			len += self.consume_decimal_digits();
			flags &= 0b011;
			flags |= 0b001;
		}
		if matches!(self.peek_nth(0), 'e' | 'E')
			&& (self.peek_nth(1).is_ascii_digit()
				|| (matches!(self.peek_nth(1), '-' | '+') && self.peek_nth(2).is_ascii_digit()))
		{
			self.next();
			len += 1;
			if matches!(self.peek_nth(0), '-' | '+') {
				self.next();
				len += 1;
			}
			len += self.consume_decimal_digits();
			flags &= 0b011;
			flags |= 0b001;
		}
		match self.peek_nth(0) {
			'%' => {
				self.next();
				Token::new_dimension(flags, offset, len, 1, acc, DimensionUnit::Percent)
			}
			c if is_ident_start_sequence(c, self.peek_nth(1), self.peek_nth(2)) => {
				let (known_unit, unit_len) = self.consume_ident_sequence_finding_known_dimension();
				Token::new_dimension(flags, offset, len, unit_len, acc, known_unit)
			}
			_ => Token::new_number(flags, offset, len, acc),
		}
	}

	#[must_use]
	fn consume_hash_token(&mut self, offset: u32) -> Token {
		self.next();
		let hash_flags = if is_ident(self.peek_nth(0)) { 0b010 } else { 0b000 };
		let (flags, len) = self.consume_ident_sequence();
		Token::new(Kind::Hash, (flags & 0b101) | hash_flags, offset, len + 1)
	}

	#[must_use]
	fn consume_decimal_digits_returning_val(&mut self, acc: u32) -> (u32, u32) {
		let mut acc = acc;
		let mut len = 0;
		while self.peek_nth(0).is_ascii_digit() {
			acc = (acc * 10) + (self.next().unwrap() as u32 - 48);
			len += 1;
		}
		(acc, len)
	}

	#[must_use]
	fn consume_decimal_digits(&mut self) -> u32 {
		let mut len = 0;
		while self.peek_nth(0).is_ascii_digit() {
			self.next();
			len += 1;
		}
		len
	}

	#[must_use]
	fn consume_ident_sequence_finding_url_keyword(&mut self) -> (u8, u32, bool) {
		let mut flags = 0b000;
		let mut len = 0;
		let mut is_url_like_keyword = true;
		let mut i = 0;
		loop {
			let c = self.peek_nth(0);
			if i == 0 && c == '-' {
				self.next();
				i += 1;
				len += 1;
				if self.peek_nth(0) == '-' {
					self.next();
					i += 1;
					len += 1;
					flags |= 0b010;
				}
			} else if is_ident_ascii_lower(c) {
				if (i == 0 && c != 'u') || (i == 1 && c != 'r') || (i == 2 && c != 'l') {
					is_url_like_keyword = false;
				}
				self.next();
				len += 1;
				i += 1;
			} else if is_ident(c) {
				if (i == 0 && c != 'U') || (i == 1 && c != 'R') || (i == 2 && c != 'L') {
					is_url_like_keyword = false;
				}
				self.next();
				len += c.len_utf8() as u32;
				i += 1;
				flags |= 0b001;
			} else if is_url_like_keyword && is_escape_sequence(c, self.peek_nth(1)) {
				let char_inspect = self.clone();
				self.next();
				flags |= 0b100;
				let esc_len = self.consume_escape_sequence();
				len += esc_len;
				let str = char_inspect.as_str()[0..esc_len as usize].trim();
				if (i == 0 && !(str == "\\75" || str == "\\55"))
					|| (i == 1 && !(str == "\\72" || str == "\\52"))
					|| (i == 2 && !(str == "\\6c" || str == "\\4c"))
				{
					is_url_like_keyword = false
				}
				i += 1;
			} else if is_escape_sequence(c, self.peek_nth(1)) {
				self.next();
				flags |= 0b100;
				len += self.consume_escape_sequence();
				i += 1;
			} else if c == '\0' && self.next().is_some() {
				// Set the escape flag to ensure \0s get replaced
				flags |= 0b100;
				len += 1;
				is_url_like_keyword = false;
			} else {
				break;
			}
		}
		(flags, len, is_url_like_keyword)
	}

	#[must_use]
	fn consume_ident_like_token(&mut self, offset: u32) -> Token {
		let (flags, mut len, is_url_like_keyword) = self.consume_ident_sequence_finding_url_keyword();
		if self.peek_nth(0) == '(' {
			self.next();
			len += 1;
			let token = Token::new(Kind::Function, flags, offset, len);
			if is_url_like_keyword {
				let mut chars = self.clone();
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
			return token;
		}
		Token::new(Kind::Ident, flags, offset, len)
	}

	#[must_use]
	fn consume_string_token(&mut self, offset: u32) -> Token {
		let delimiter = self.next().unwrap();
		let mut len = 1;
		let mut flags = (delimiter == '"') as u8;
		loop {
			match self.peek_nth(0) {
				c if is_newline(c) => {
					return Token::new(Kind::BadString, flags, offset, len);
				}
				EOF => {
					if self.next().is_some() {
						// Set the escape flag to ensure \0s get replaced
						flags |= 0b100;
						len += 1;
					} else {
						return Token::new(Kind::String, flags, offset, len);
					}
				}
				c @ ('"' | '\'') => {
					self.next();
					len += 1;
					if c == delimiter {
						flags |= 0b010;
						return Token::new(Kind::String, flags, offset, len);
					}
				}
				c @ '\\' => {
					self.next();
					flags |= 0b100;
					match self.peek_nth(0) {
						EOF => {
							len += 1;
							return Token::new(Kind::String, flags, offset, len);
						}
						p if is_newline(p) => {
							len += self.consume_newline() + 1;
						}
						p if is_escape_sequence(c, p) => {
							len += self.consume_escape_sequence();
						}
						_ => {
							return Token::new(Kind::BadString, flags, offset, len);
						}
					}
				}
				c => {
					self.next();
					len += c.len_utf8() as u32;
				}
			}
		}
	}

	#[must_use]
	fn is_number_start(&mut self) -> bool {
		self.peek_nth(0).is_ascii_digit()
			|| (is_sign(self.peek_nth(0))
				&& (self.peek_nth(1).is_ascii_digit() || self.peek_nth(1) == '.' && self.peek_nth(2).is_ascii_digit()))
			|| (self.peek_nth(0) == '.' && self.peek_nth(1).is_ascii_digit())
	}
}

impl<'a> Lexer<'a> {
	#[must_use]
	pub(crate) fn read_next_token(&mut self) -> Token {
		let offset = self.current_token.end_offset();
		if self.source.len() as u32 == offset {
			return Token::new(Kind::Eof, 0, offset, 0);
		}
		let mut chars = self.source[offset as usize..].chars();
		let c = chars.peek_nth(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let kind = SINGLE_CHAR_TOKENS[size];
			if kind != Kind::Eof {
				chars.next();
				return Token::new(kind, 0, offset, c as u32);
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return chars.consume_ident_like_token(offset);
			}
		}
		match c {
			'\0' => {
				// https://drafts.csswg.org/css-syntax-3/#input-preprocessing
				// The input stream consists of the filtered code points pushed into it as the input byte stream is decoded.
				// To filter code points from a stream of (unfiltered) code points input:
				//  Replace any U+0000 NULL or surrogate code points in input with U+FFFD REPLACEMENT CHARACTER (�).
				//
				if !chars.is_last()
					&& is_ident_start_sequence(REPLACEMENT_CHARACTER, chars.peek_nth(1), chars.peek_nth(2))
				{
					chars.consume_ident_like_token(offset)
				} else if chars.next().is_some() {
					Token::new(Kind::Delim, 1, offset, REPLACEMENT_CHARACTER as u32)
				} else {
					Token::new(Kind::Eof, 0, offset, 0)
				}
			}
			// Whitespace Range
			c if is_whitespace(c) => {
				let len = chars.consume_whitespace();
				Token::new(Kind::Whitespace, 0, offset, len)
			}
			// Quote Range
			c if is_quote(c) => chars.consume_string_token(offset),
			// Digit Range
			c if c.is_ascii_digit() => chars.consume_numeric_token(offset),
			// Sign Range
			'-' => {
				if chars.peek_nth(1) == '-' && chars.peek_nth(2) == '>' {
					chars.next();
					chars.next();
					chars.next();
					return Token::new(Kind::CdcOrCdo, 1, offset, 3);
				}
				if is_ident_start_sequence(c, chars.peek_nth(1), chars.peek_nth(2)) {
					return chars.consume_ident_like_token(offset);
				}
				if chars.is_number_start() {
					return chars.consume_numeric_token(offset);
				}
				chars.next();
				Token::new(Kind::Delim, 1, offset, '-' as u32)
			}
			// Dot or Plus
			'.' | '+' => {
				if chars.is_number_start() {
					return chars.consume_numeric_token(offset);
				}
				chars.next();
				Token::new(Kind::Delim, 1, offset, c as u32)
			}
			// Less Than
			'<' => {
				chars.next();
				if chars.peek_nth(0) == '!' && chars.peek_nth(1) == '-' && chars.peek_nth(2) == '-' {
					chars.next();
					chars.next();
					chars.next();
					return Token::new(Kind::CdcOrCdo, 0, offset, 4);
				}
				Token::new(Kind::Delim, 1, offset, '<' as u32)
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(chars.peek_nth(1)) || is_escape_sequence(chars.peek_nth(1), chars.peek_nth(2)) {
					chars.consume_hash_token(offset)
				} else {
					chars.next();
					Token::new(Kind::Delim, 1, offset, '#' as u32)
				}
			}
			// Commercial At
			'@' => {
				chars.next();
				if is_ident_start_sequence(chars.peek_nth(0), chars.peek_nth(1), chars.peek_nth(2)) {
					let (flags, len) = chars.consume_ident_sequence();
					return Token::new(Kind::AtKeyword, flags, offset, len + 1);
				}
				Token::new(Kind::Delim, 1, offset, '@' as u32)
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, chars.peek_nth(1)) {
					return chars.consume_ident_like_token(offset);
				}
				chars.next();
				Token::new(Kind::Delim, 1, offset, '\\' as u32)
			}
			// Solidus
			'/' => match chars.peek_nth(1) {
				'*' => {
					chars.next();
					chars.next();
					let mut len = 2;
					while let Some(c) = chars.next() {
						len += c.len_utf8() as u32;
						if c == '*' && chars.peek_nth(0) == '/' {
							chars.next();
							len += 1;
							break;
						}
					}
					Token::new(Kind::Comment, 0, offset, len)
				}
				_ => {
					chars.next();
					Token::new(Kind::Delim, 1, offset, '/' as u32)
				}
			},
			c if is_ident_start(c) => chars.consume_ident_like_token(offset),
			c => {
				chars.next();
				Token::new(Kind::Delim, c.len_utf8() as u8, offset, c as u32)
			}
		}
	}
}
