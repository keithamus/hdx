use hdx_syntax::{
	identifier::{is_ident, is_ident_ascii_lower, is_ident_ascii_start, is_ident_start, is_ident_start_sequence},
	is_escape_sequence, is_newline, is_quote, is_sign, is_whitespace,
	url::is_non_printable,
	CR, EOF, FF, LF, SPACE, TAB,
};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

use crate::{
	constants::SINGLE_CHAR_TOKENS, CommentStyle, DimensionUnit, Feature, Lexer, QuoteStyle, Token, Whitespace,
};

trait CharsConsumer {
	fn is_last(&self) -> bool;

	fn peek_nth(&self, n: usize) -> char;

	#[must_use]
	fn consume_newline(&mut self) -> u32;

	#[must_use]
	fn consume_same(&mut self, char: char) -> u32;

	#[must_use]
	fn consume_whitespace(&mut self) -> (u32, Whitespace);

	#[must_use]
	fn consume_ident_sequence(&mut self) -> (u32, bool, bool, bool);

	#[must_use]
	fn consume_ident_sequence_finding_known_dimension(&mut self) -> (DimensionUnit, u32);

	#[must_use]
	fn consume_ident_sequence_finding_url_keyword(&mut self) -> (u32, bool, bool, bool, bool);

	#[must_use]
	fn consume_escape_sequence(&mut self) -> u32;

	#[must_use]
	fn consume_url_sequence(&mut self, len: u32, ident_escaped: bool) -> Token;

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, len: u32) -> Token;

	#[must_use]
	fn consume_numeric_token(&mut self) -> Token;

	#[must_use]
	fn consume_hash_token(&mut self) -> Token;

	#[must_use]
	fn consume_ident_like_token(&mut self) -> Token;

	#[must_use]
	fn consume_string_token(&mut self) -> Token;

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
	fn consume_same(&mut self, char: char) -> u32 {
		let mut i = 0;
		while self.peek_nth(0) == char {
			self.next();
			i += 1;
		}
		i
	}

	#[must_use]
	fn consume_whitespace(&mut self) -> (u32, Whitespace) {
		let mut i = 0;
		let mut style = Whitespace::none();
		while is_whitespace(self.peek_nth(0)) {
			let c = self.next().unwrap();
			if c == ' ' {
				style |= Whitespace::Space;
			} else if c == '\t' {
				style |= Whitespace::Tab;
			} else {
				style |= Whitespace::Newline;
			}
			i += 1;
		}
		(i, style)
	}

	#[must_use]
	fn consume_ident_sequence(&mut self) -> (u32, bool, bool, bool) {
		let mut dashed_ident = false;
		let mut contains_non_lower_ascii = false;
		let mut contains_escape = false;
		let mut len = 0;
		loop {
			let c = self.peek_nth(0);
			if len == 0 && c == '-' {
				self.next();
				len += 1;
				if self.peek_nth(0) == '-' {
					self.next();
					len += 1;
					dashed_ident = true;
				}
			} else if is_ident_ascii_lower(c) || c == '-' || c.is_ascii_digit() {
				self.next();
				len += 1;
			} else if is_ident(c) {
				self.next();
				len += c.len_utf8() as u32;
				contains_non_lower_ascii = true;
			} else if is_escape_sequence(c, self.peek_nth(1)) {
				self.next();
				contains_escape = true;
				len += self.consume_escape_sequence();
			} else if c == '\0' && self.next().is_some() {
				// Set the escape flag to ensure \0s get replaced
				contains_escape = true;
				len += 1;
			} else {
				break;
			}
		}
		(len, contains_non_lower_ascii, dashed_ident, contains_escape)
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
			// Deg, Dpcm, Dpi, Dppx, Dvb, Dvh, Dvi, Dvw, Dvmax, Dvmin
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
				if c == 'b' || c == 'B' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvb
				} else if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvh
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvi
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Dvw
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
							unit = DimensionUnit::Dvmax
						}
					} else if c == 'i' || c == 'I' {
						self.next();
						len += 1;
						let c = self.peek_nth(0);
						if c == 'n' || c == 'N' {
							self.next();
							len += 1;
							unit = DimensionUnit::Dvmin
						}
					}
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
			if c == 'h' || c == 'H' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'z' || c == 'Z' {
					self.next();
					len += 1;
					unit = DimensionUnit::Khz
				}
			}
		} else if c == 'l' || c == 'L' {
			// Lh, Lvb, Lvi, Lvh, Lvw, Lvmax, Lvmin
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
				if c == 'b' || c == 'B' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvb
				} else if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvh
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvi
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Lvw
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
							unit = DimensionUnit::Lvmax
						}
					} else if c == 'i' || c == 'I' {
						self.next();
						len += 1;
						let c = self.peek_nth(0);
						if c == 'n' || c == 'N' {
							self.next();
							len += 1;
							unit = DimensionUnit::Lvmin
						}
					}
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
			// S, Svb, Svi, Svh, Svw, Svmax, Svmin
			self.next();
			len += 1;
			unit = DimensionUnit::S;
			let c = self.peek_nth(0);
			if c == 'v' || c == 'V' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'b' || c == 'B' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svb
				} else if c == 'h' || c == 'H' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svh
				} else if c == 'i' || c == 'I' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svi
				} else if c == 'w' || c == 'W' {
					self.next();
					len += 1;
					unit = DimensionUnit::Svw
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
							unit = DimensionUnit::Svmax
						}
					} else if c == 'i' || c == 'I' {
						self.next();
						len += 1;
						let c = self.peek_nth(0);
						if c == 'n' || c == 'N' {
							self.next();
							len += 1;
							unit = DimensionUnit::Svmin
						}
					}
				}
			}
		} else if c == 't' || c == 'T' {
			// Turn,
			// Vb, Vh, Vi, Vmax, Vmin, Vw,
			self.next();
			len += 1;
			let c = self.peek_nth(0);
			if c == 'u' || c == 'U' {
				self.next();
				len += 1;
				let c = self.peek_nth(0);
				if c == 'r' || c == 'R' {
					self.next();
					len += 1;
					let c = self.peek_nth(0);
					if c == 'n' || c == 'N' {
						self.next();
						len += 1;
						unit = DimensionUnit::Turn
					}
				}
			}
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
		let (rest_len, _, _, _) = self.consume_ident_sequence();
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
	fn consume_url_sequence(&mut self, leading_len: u32, ident_escaped: bool) -> Token {
		let mut len = leading_len;
		let mut trailing_len = 0;
		let mut contains_escape = ident_escaped;
		let mut ends_with_paren = false;
		let (whitespace_count, _) = self.consume_whitespace();
		if whitespace_count > 0 {
			len += whitespace_count;
		}
		loop {
			let c = self.peek_nth(0);
			match c {
				')' => {
					self.next();
					len += 1;
					trailing_len += 1;
					ends_with_paren = true;
					break;
				}
				EOF => {
					break;
				}
				_ if is_whitespace(c) => {
					trailing_len += self.consume_whitespace().0;
					len += trailing_len;
					// Consider trailing whitespace as escape to allow the string
					// parser to consume characters one-by-one
					contains_escape = true;
					match self.peek_nth(0) {
						')' => {
							self.next();
							len += 1;
							trailing_len += 1;
							ends_with_paren = true;
							break;
						}
						EOF => {
							break;
						}
						_ => {
							return self.consume_remnants_of_bad_url(len);
						}
					};
				}
				'\'' | '"' | '(' => {
					return self.consume_remnants_of_bad_url(len);
				}
				_ if is_non_printable(c) => {
					return self.consume_remnants_of_bad_url(len);
				}
				'\\' => {
					if is_escape_sequence(c, self.peek_nth(1)) {
						self.next();
						len += self.consume_escape_sequence();
						contains_escape = true;
					} else {
						return self.consume_remnants_of_bad_url(len);
					}
				}
				c => {
					self.next();
					len += c.len_utf8() as u32;
				}
			}
		}
		Token::new_url(
			ends_with_paren,
			whitespace_count > 0,
			contains_escape,
			leading_len + whitespace_count,
			trailing_len,
			len,
		)
	}

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, len: u32) -> Token {
		let mut len = len;
		while let Some(ch) = self.next() {
			match ch {
				')' => {
					len += 1;
					break;
				}
				'\\' => {
					if is_escape_sequence(ch, self.peek_nth(0)) {
						len += self.consume_escape_sequence();
					} else if let Some(ch) = self.next() {
						len += ch.len_utf8() as u32 + 1;
					}
				}
				_ => {
					len += ch.len_utf8() as u32;
				}
			}
		}
		Token::new_bad_url(len)
	}

	#[must_use]
	fn consume_numeric_token(&mut self) -> Token {
		let mut numchars = self.clone();
		let c = numchars.next().unwrap();
		let mut num_len = 1;
		let mut is_float = c == '.';
		let has_sign = is_sign(c);
		while numchars.peek_nth(0).is_ascii_digit() {
			num_len += 1;
			numchars.next();
		}
		if !is_float && numchars.peek_nth(0) == '.' && numchars.peek_nth(1).is_ascii_digit() {
			numchars.next();
			num_len += 1;
			while numchars.peek_nth(0).is_ascii_digit() {
				num_len += 1;
				numchars.next();
			}
			is_float = true;
		}
		if matches!(numchars.peek_nth(0), 'e' | 'E')
			&& (numchars.peek_nth(1).is_ascii_digit()
				|| (matches!(numchars.peek_nth(1), '-' | '+') && numchars.peek_nth(2).is_ascii_digit()))
		{
			numchars.next();
			num_len += 1;
			let c = numchars.peek_nth(0);
			if matches!(c, '-' | '+') {
				numchars.next();
				num_len += 1;
			}
			while numchars.peek_nth(0).is_ascii_digit() {
				num_len += 1;
				numchars.next();
			}
			is_float = true;
		}
		let value = self.as_str()[0..num_len].parse::<f32>().unwrap();
		self.nth(num_len - 1);
		match self.peek_nth(0) {
			'%' => {
				self.next();
				Token::new_dimension(is_float, has_sign, num_len as u32, 1, value, DimensionUnit::Percent)
			}
			c if is_ident_start_sequence(c, self.peek_nth(1), self.peek_nth(2)) => {
				let (known_unit, unit_len) = self.consume_ident_sequence_finding_known_dimension();
				Token::new_dimension(is_float, has_sign, num_len as u32, unit_len, value, known_unit)
			}
			_ => Token::new_number(is_float, has_sign, num_len as u32, value),
		}
	}

	#[must_use]
	fn consume_hash_token(&mut self) -> Token {
		self.next();
		let first_is_ascii = is_ident(self.peek_nth(0));
		let (len, contains_non_lower_ascii, _, contains_escape) = self.consume_ident_sequence();
		Token::new_hash(contains_non_lower_ascii, first_is_ascii, contains_escape, len + 1)
	}

	#[must_use]
	fn consume_ident_sequence_finding_url_keyword(&mut self) -> (u32, bool, bool, bool, bool) {
		let mut dashed_ident = false;
		let mut contains_non_lower = false;
		let mut contains_escape = false;
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
					dashed_ident = true;
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
				contains_non_lower = true;
			} else if is_url_like_keyword && is_escape_sequence(c, self.peek_nth(1)) {
				let char_inspect = self.clone();
				self.next();
				contains_escape = true;
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
				contains_escape = true;
				len += self.consume_escape_sequence();
				i += 1;
			} else if c == '\0' && self.next().is_some() {
				// Set the escape flag to ensure \0s get replaced
				contains_escape = true;
				len += 1;
				is_url_like_keyword = false;
			} else {
				break;
			}
		}
		(len, contains_non_lower, dashed_ident, contains_escape, is_url_like_keyword)
	}

	#[must_use]
	fn consume_ident_like_token(&mut self) -> Token {
		let (mut len, contains_non_lower_ascii, dashed, contains_escape, is_url_like_keyword) =
			self.consume_ident_sequence_finding_url_keyword();
		if self.peek_nth(0) == '(' {
			self.next();
			len += 1;
			let token = Token::new_function(contains_non_lower_ascii, dashed, contains_escape, len);
			if is_url_like_keyword {
				let mut chars = self.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					return self.consume_url_sequence(len, contains_escape);
				}
			}
			return token;
		}
		Token::new_ident(contains_non_lower_ascii, dashed, contains_escape, len)
	}

	#[must_use]
	fn consume_string_token(&mut self) -> Token {
		let delimiter = self.next().unwrap();
		let quotes = if delimiter == '"' { QuoteStyle::Double } else { QuoteStyle::Single };
		let mut contains_escape = false;
		let mut len = 1;
		loop {
			match self.peek_nth(0) {
				c if is_newline(c) => {
					return Token::new_bad_string(len);
				}
				EOF => {
					if self.next().is_some() {
						// Set the escape flag to ensure \0s get replaced
						contains_escape = true;
						len += 1;
					} else {
						return Token::new_string(quotes, false, contains_escape, len);
					}
				}
				c @ ('"' | '\'') => {
					self.next();
					len += 1;
					if c == delimiter {
						return Token::new_string(quotes, true, contains_escape, len);
					}
				}
				c @ '\\' => {
					self.next();
					contains_escape = true;
					match self.peek_nth(0) {
						EOF => {
							len += 1;
							return Token::new_string(quotes, false, contains_escape, len);
						}
						p if is_newline(p) => {
							len += self.consume_newline() + 1;
						}
						p if is_escape_sequence(c, p) => {
							len += self.consume_escape_sequence();
						}
						_ => return Token::new_bad_string(len),
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
	pub(crate) fn read_next_token(&mut self, offset: u32) -> Token {
		if self.source.len() as u32 == offset {
			return Token::EOF;
		}
		let mut chars = self.source[offset as usize..].chars();
		let c = chars.peek_nth(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let token = SINGLE_CHAR_TOKENS[size];
			if token != Token::EOF {
				return token;
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return chars.consume_ident_like_token();
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
					chars.consume_ident_like_token()
				} else if chars.next().is_some() {
					Token::new_delim(REPLACEMENT_CHARACTER)
				} else {
					Token::EOF
				}
			}
			c if is_whitespace(c) && !self.features.contains(Feature::SeparateWhitespace) => {
				let (len, style) = chars.consume_whitespace();
				Token::new_whitespace(style, len)
			}
			// Whitespace Range
			TAB => Token::new_whitespace(Whitespace::Tab, chars.consume_same(TAB)),
			SPACE => Token::new_whitespace(Whitespace::Space, chars.consume_same(SPACE)),
			LF | CR | FF => {
				// https://drafts.csswg.org/css-syntax/#input-preprocessing
				//  Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED
				//  (FF) code points, or pairs of U+000D CARRIAGE RETURN (CR) followed by
				//  U+000A LINE FEED (LF) in input by a single U+000A LINE FEED (LF) code
				//  point.
				let mut len = 0;
				loop {
					let c = chars.peek_nth(0);
					if !matches!(c, LF | CR | FF) {
						break;
					}
					chars.next();
					len += 1;
				}
				Token::new_whitespace(Whitespace::Newline, len)
			}
			// Quote Range
			c if is_quote(c) => chars.consume_string_token(),
			// Digit Range
			c if c.is_ascii_digit() => chars.consume_numeric_token(),
			// Sign Range
			'-' => {
				if chars.peek_nth(1) == '-' && chars.peek_nth(2) == '>' {
					chars.next();
					chars.next();
					chars.next();
					return Token::CDC;
				}
				if is_ident_start_sequence(c, chars.peek_nth(1), chars.peek_nth(2)) {
					return chars.consume_ident_like_token();
				}
				if chars.is_number_start() {
					return chars.consume_numeric_token();
				}
				chars.next();
				Token::DASH
			}
			// Dot or Plus
			'.' | '+' => {
				if chars.is_number_start() {
					return chars.consume_numeric_token();
				}
				chars.next();
				Token::new_delim(c)
			}
			// Less Than
			'<' => {
				chars.next();
				if chars.peek_nth(0) == '!' && chars.peek_nth(1) == '-' && chars.peek_nth(2) == '-' {
					chars.next();
					chars.next();
					chars.next();
					return Token::CDO;
				}
				Token::LESS_THAN
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(chars.peek_nth(1)) || is_escape_sequence(chars.peek_nth(1), chars.peek_nth(2)) {
					chars.consume_hash_token()
				} else {
					chars.next();
					Token::HASH
				}
			}
			// Commercial At
			'@' => {
				chars.next();
				if is_ident_start_sequence(chars.peek_nth(0), chars.peek_nth(1), chars.peek_nth(2)) {
					let (len, contains_non_lower_ascii, dashed, contains_escape) = chars.consume_ident_sequence();
					return Token::new_atkeyword(contains_non_lower_ascii, dashed, contains_escape, len + 1);
				}
				Token::AT
			}
			// Reverse Solidus
			'\\' => {
				if is_escape_sequence(c, chars.peek_nth(1)) {
					return chars.consume_ident_like_token();
				}
				chars.next();
				Token::BACKSLASH
			}
			// Solidus
			'/' => match chars.peek_nth(1) {
				'*' => {
					chars.next();
					chars.next();
					let mut len = 2;
					let comment_style = match chars.peek_nth(0) {
						'*' if chars.peek_nth(1) != '/' => CommentStyle::BlockStar,
						'#' => CommentStyle::BlockPound,
						'!' => CommentStyle::BlockBang,
						'-' | '=' => CommentStyle::BlockHeading,
						_ => CommentStyle::Block,
					};
					while let Some(c) = chars.next() {
						len += c.len_utf8() as u32;
						if c == '*' && chars.peek_nth(0) == '/' {
							chars.next();
							len += 1;
							break;
						}
					}
					Token::new_comment(comment_style, len)
				}
				'/' if self.features.intersects(Feature::SingleLineComments) => {
					chars.next();
					chars.next();
					let mut len = 2;
					let comment_style = match chars.peek_nth(0) {
						'*' => CommentStyle::SingleStar,
						'!' => CommentStyle::SingleBang,
						_ => CommentStyle::Single,
					};
					while !matches!(chars.peek_nth(0), LF | CR | FF | EOF) {
						chars.next();
						len += 1;
					}
					Token::new_comment(comment_style, len)
				}
				_ => {
					chars.next();
					Token::SLASH
				}
			},
			c if is_ident_start(c) => chars.consume_ident_like_token(),
			c => {
				chars.next();
				Token::new_delim(c)
			}
		}
	}
}
