use crate::{
	CommentStyle, Feature, Lexer, QuoteStyle, Token, Whitespace,
	constants::SINGLE_CHAR_TOKENS,
	scan::{scan_bad_url, scan_ident, scan_ident_non_lower, scan_line_comment, scan_string, scan_url},
	small_str_buf::SmallStrBuf,
	syntax::{
		CR, EOF, FF, LF, SPACE, TAB,
		identifier::{is_ident, is_ident_ascii_start, is_ident_byte, is_ident_start, is_ident_start_sequence},
		is_escape_sequence, is_non_printable, is_quote, is_url_ident, is_whitespace,
		tables::{ASCII_NEWLINE, ASCII_WHITESPACE},
	},
};
use atom_set::DynAtomSet;
use bytescan::scan_byte;
use std::char::REPLACEMENT_CHARACTER;

// 7 makes size_of::<SmallStrBuf<8>>() == size_of::<usize>()
const MAX_SMALL_IDENT_SIZE: usize = 7;

/// A byte-position cursor over a `&[u8]` slice (which must be valid UTF-8).
struct ByteCursor<'a> {
	bytes: &'a [u8],
	pos: usize,
}

impl<'a> ByteCursor<'a> {
	#[inline(always)]
	fn new(bytes: &'a [u8]) -> Self {
		Self { bytes, pos: 0 }
	}

	/// Peek at the byte at offset `n` from current position.
	/// Returns 0 (NUL) if past end — this maps to EOF in the CSS spec.
	#[inline(always)]
	fn peek_byte(&self, n: usize) -> u8 {
		let idx = self.pos + n;
		if idx < self.bytes.len() {
			// SAFETY: we just checked bounds
			unsafe { *self.bytes.get_unchecked(idx) }
		} else {
			0
		}
	}

	/// Peek the next char (decoding UTF-8 if needed). Returns EOF ('\0') if at end.
	#[inline(always)]
	fn peek_char(&self) -> char {
		self.peek_char_at(0)
	}

	/// Peek a char at offset `n` bytes from current position.
	#[inline(always)]
	fn peek_char_at(&self, n: usize) -> char {
		let idx = self.pos + n;
		if idx >= self.bytes.len() {
			return EOF;
		}
		let b = unsafe { *self.bytes.get_unchecked(idx) };
		if b < 128 {
			b as char
		} else {
			// Decode multi-byte UTF-8
			self.decode_non_ascii_at(idx)
		}
	}

	/// Decode a non-ASCII char starting at `idx`. The byte at `idx` is >= 128.
	fn decode_non_ascii_at(&self, idx: usize) -> char {
		// SAFETY: self.bytes is valid UTF-8
		let remaining = unsafe { std::str::from_utf8_unchecked(&self.bytes[idx..]) };
		remaining.chars().next().unwrap_or(EOF)
	}

	/// Peek the next two chars
	#[inline(always)]
	fn peek2(&self) -> (char, char) {
		let c0 = self.peek_char_at(0);
		let c1 = self.peek_char_at(c0.len_utf8());
		(c0, c1)
	}

	/// Peek the next three chars
	#[inline(always)]
	fn peek3(&self) -> (char, char, char) {
		let c0 = self.peek_char_at(0);
		let off1 = c0.len_utf8();
		let c1 = self.peek_char_at(off1);
		let c2 = self.peek_char_at(off1 + c1.len_utf8());
		(c0, c1, c2)
	}

	/// Advance by one byte (for ASCII). Returns the byte.
	#[inline(always)]
	fn advance_byte(&mut self) -> u8 {
		debug_assert!(self.pos < self.bytes.len());
		let b = unsafe { *self.bytes.get_unchecked(self.pos) };
		self.pos += 1;
		b
	}

	/// Advance by `n` bytes.
	#[inline(always)]
	fn advance_n(&mut self, n: usize) {
		self.pos += n;
	}

	/// Advance past one char, return it. Returns None if at end.
	#[inline(always)]
	fn next_char(&mut self) -> Option<char> {
		if self.pos >= self.bytes.len() {
			return None;
		}
		let b = unsafe { *self.bytes.get_unchecked(self.pos) };
		if b < 128 {
			self.pos += 1;
			Some(b as char)
		} else {
			let remaining = unsafe { std::str::from_utf8_unchecked(&self.bytes[self.pos..]) };
			let c = remaining.chars().next()?;
			self.pos += c.len_utf8();
			Some(c)
		}
	}

	/// Check if at end (no more bytes)
	#[inline(always)]
	fn at_end(&self) -> bool {
		self.pos >= self.bytes.len()
	}

	/// Check if there's exactly one or zero bytes remaining.
	#[inline(always)]
	fn is_last(&self) -> bool {
		self.pos >= self.bytes.len()
	}

	/// Return the remaining bytes as a &str (valid since source is UTF-8)
	#[inline(always)]
	fn remaining_str(&self) -> &'a str {
		unsafe { std::str::from_utf8_unchecked(&self.bytes[self.pos..]) }
	}

	/// Return the remaining bytes slice
	#[inline(always)]
	fn remaining_bytes(&self) -> &'a [u8] {
		&self.bytes[self.pos..]
	}

	fn consume_newline(&mut self) -> u32 {
		let b = self.peek_byte(0);
		if b == b'\r' {
			self.pos += 1;
			if self.peek_byte(0) == b'\n' {
				self.pos += 1;
				return 2;
			}
			return 1;
		}
		// LF or FF
		if self.pos < self.bytes.len() {
			self.pos += 1;
		}
		1
	}

	fn consume_same(&mut self, byte: u8) -> u32 {
		let mut i: u32 = 0;
		while self.peek_byte(0) == byte {
			self.pos += 1;
			i += 1;
		}
		i
	}

	fn consume_whitespace(&mut self) -> (u32, Whitespace) {
		let start = self.pos;
		let mut style = Whitespace::none();
		while let Some(&b) = self.bytes.get(self.pos) {
			style |= match b {
				b' ' => Whitespace::Space,
				b'\t' => Whitespace::Tab,
				b'\n' | b'\r' | 0x0C => Whitespace::Newline,
				_ => break,
			};
			self.pos += 1;
		}
		((self.pos - start) as u32, style)
	}

	fn consume_ident_sequence(&mut self, atoms: &dyn DynAtomSet) -> (u32, bool, bool, bool, u32, bool) {
		let mut dashed_ident = false;
		let mut contains_non_lower_ascii = false;
		let mut contains_escape = false;

		let remaining = self.remaining_bytes();
		if !remaining.is_empty() {
			let bytes = remaining;
			let end = bytes.len();
			let mut i = 0;
			if bytes.len() >= 2 && bytes[0] == b'-' && bytes[1] == b'-' {
				i = 2;
				dashed_ident = true;
			}
			let scan_start = i;
			let (off, upper) = scan_ident(&bytes[i..]);
			i += off;
			contains_non_lower_ascii |= upper;

			let ascii_len = (i - scan_start) as u32;
			let len = i as u32;
			if ascii_len > 0 {
				let next_byte = if i < end { bytes[i] } else { b' ' };
				if next_byte < 128 && !is_ident_byte(next_byte) && next_byte != b'\\' && next_byte != 0 {
					// SAFETY: the fast path only scans ASCII ident bytes, which are valid UTF-8.
					let ident_str = unsafe { std::str::from_utf8_unchecked(&bytes[..i]) };
					self.advance_n(i);
					let atom_bits = atoms.str_to_bits(ident_str);
					return (
						len,
						contains_non_lower_ascii,
						dashed_ident,
						false,
						atom_bits,
						ascii_len == 3 && is_url_ident(ident_str),
					);
				}
			}
		}

		let start = self.pos;
		let str = self.remaining_str();
		let mut small_ident = SmallStrBuf::<MAX_SMALL_IDENT_SIZE>::new();

		loop {
			let rest = self.remaining_str();
			let (run, non_lower) = scan_ident_non_lower(rest.as_bytes());
			small_ident.append_str(&rest[..run]);
			self.advance_n(run);
			contains_non_lower_ascii |= non_lower;
			let c = self.peek_char();
			if is_ident(c) {
				self.advance_n(c.len_utf8());
				contains_non_lower_ascii = true;
				small_ident.append(c);
			} else if c == '\\' && is_escape_sequence(c, self.peek_char_at(1)) {
				self.pos += 1;
				contains_escape = true;
				if small_ident.over_capacity() {
					self.consume_escape_sequence();
				} else {
					small_ident.append(self.parse_escape_sequence().0);
				}
			} else if c == '\0' && !self.at_end() {
				self.pos += 1;
				contains_escape = true;
				small_ident.append(REPLACEMENT_CHARACTER);
			} else {
				break;
			}
		}
		let len = (self.pos - start) as u32;
		// The ident was small enough to be fully encoded into the small_ident buffer,
		// so it should be used over the str slice as it will have parsed escape sequences
		let (is_url, atom_bits) = if let Some(ident) = small_ident.as_str() {
			(is_url_ident(ident), atoms.str_to_bits(ident))
		} else {
			// We intentionally make the small_ident buffer large enough to capture the `url` ident an unescape it,
			// so this branch would never be hit with a valid URL, we can guarantee this ident is not a URL.
			(false, atoms.str_to_bits(&str[0..len as usize]))
		};
		(len, contains_non_lower_ascii, dashed_ident, contains_escape, atom_bits, !dashed_ident && is_url)
	}

	fn consume_escape_sequence(&mut self) -> u32 {
		if let Some(c) = self.next_char() {
			if c.is_ascii_hexdigit() {
				let (_, hex_len) = self.consume_hex_escape(c);
				((hex_len + self.consume_escape_whitespace()) as u32) + 1
			} else {
				(c.len_utf8() as u32) + 1
			}
		} else {
			1
		}
	}

	fn consume_hex_escape(&mut self, first_digit: char) -> (u32, u8) {
		let mut value = first_digit.to_digit(16).unwrap();
		let mut i = 1u8;
		while i < 6 {
			let b = self.peek_byte(0);
			if b < 128 {
				if let Some(hex_value) = (b as char).to_digit(16) {
					self.pos += 1;
					value = (value << 4) | hex_value;
					i += 1;
				} else {
					break;
				}
			} else {
				break;
			}
		}
		(value, i)
	}

	fn consume_escape_whitespace(&mut self) -> u8 {
		let b = self.peek_byte(0);
		if b < 128 && ASCII_WHITESPACE.0[b as usize] {
			self.pos += 1;
			if b == b'\r' && self.peek_byte(0) == b'\n' {
				self.pos += 1;
				2
			} else {
				1
			}
		} else {
			0
		}
	}

	fn parse_escape_sequence(&mut self) -> (char, u8) {
		if let Some(c) = self.next_char() {
			if !c.is_ascii_hexdigit() {
				return (c, c.len_utf8() as u8);
			}
			let (value, hex_len) = self.consume_hex_escape(c);
			let ws_len = self.consume_escape_whitespace();
			(codepoint_to_char(value), hex_len + ws_len)
		} else {
			(REPLACEMENT_CHARACTER, 0)
		}
	}

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
			let b = self.peek_byte(0);
			match b {
				b')' => {
					self.pos += 1;
					len += 1;
					trailing_len += 1;
					ends_with_paren = true;
					break;
				}
				0 if self.at_end() => {
					break;
				}
				b if b < 128 && ASCII_WHITESPACE.0[b as usize] => {
					trailing_len += self.consume_whitespace().0;
					len += trailing_len;
					contains_escape = true;
					let b2 = self.peek_byte(0);
					match b2 {
						b')' => {
							self.pos += 1;
							len += 1;
							trailing_len += 1;
							ends_with_paren = true;
							break;
						}
						0 if self.at_end() => {
							break;
						}
						_ => {
							return self.consume_remnants_of_bad_url(len);
						}
					}
				}
				b'\'' | b'"' | b'(' => {
					return self.consume_remnants_of_bad_url(len);
				}
				b'\\' => {
					let c2 = self.peek_char_at(1);
					if is_escape_sequence('\\', c2) {
						self.pos += 1;
						len += self.consume_escape_sequence();
						contains_escape = true;
					} else {
						return self.consume_remnants_of_bad_url(len);
					}
				}
				_ => {
					// Bulk scan: find the next byte that needs per-byte handling.
					let remaining = self.remaining_bytes();
					let safe_end = scan_url(remaining);
					if safe_end > 0 {
						self.advance_n(safe_end);
						len += safe_end as u32;
					} else {
						// First byte is a rare sentinel; handle one byte
						let c = self.peek_char();
						if is_non_printable(c) {
							return self.consume_remnants_of_bad_url(len);
						}
						let clen = c.len_utf8();
						self.advance_n(clen);
						len += clen as u32;
					}
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

	fn consume_remnants_of_bad_url(&mut self, len: u32) -> Token {
		let mut len = len;
		loop {
			let remaining = self.remaining_bytes();
			if remaining.is_empty() {
				break;
			}
			// Bulk-skip to the next ')' or '\\' — the only bytes that need special handling.
			let offset = scan_bad_url(remaining);
			if offset == remaining.len() {
				// No ')' or '\\' in remaining input — consume it all.
				len += remaining.len() as u32;
				self.advance_n(remaining.len());
				break;
			}
			// Skip everything before the sentinel.
			if offset > 0 {
				self.advance_n(offset);
				len += offset as u32;
			}
			let b = self.peek_byte(0);
			if b == b')' {
				self.pos += 1;
				len += 1;
				break;
			}
			// b == b'\\'
			self.pos += 1;
			let next = self.peek_char();
			if is_escape_sequence('\\', next) {
				len += self.consume_escape_sequence();
			} else if let Some(ch2) = self.next_char() {
				len += ch2.len_utf8() as u32 + 1;
			}
		}
		Token::new_bad_url(len)
	}

	fn consume_numeric_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		let bytes = self.remaining_bytes();

		let mut num_len: usize = 1;
		let first_byte = bytes[0];
		let mut is_float = first_byte == b'.';
		let has_sign = first_byte == b'+' || first_byte == b'-';

		// Scan integer part
		let mut i: usize = 1;
		while i < bytes.len() && bytes[i].is_ascii_digit() {
			num_len += 1;
			i += 1;
		}

		// Check for decimal point + fractional part
		if !is_float && i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
			num_len += 1;
			i += 1;
			while i < bytes.len() && bytes[i].is_ascii_digit() {
				num_len += 1;
				i += 1;
			}
			is_float = true;
		}

		// Check for exponent part
		if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
			let next_byte = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
			let next_next_byte = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

			if next_byte.is_ascii_digit() || (matches!(next_byte, b'-' | b'+') && next_next_byte.is_ascii_digit()) {
				num_len += 1;
				i += 1;
				if matches!(bytes[i], b'-' | b'+') {
					num_len += 1;
					i += 1;
				}
				while i < bytes.len() && bytes[i].is_ascii_digit() {
					num_len += 1;
					i += 1;
				}
				is_float = true;
			}
		}

		let value = if !is_float {
			// Fast path for integers: compute value inline without str::parse
			fast_parse_integer(bytes, has_sign, num_len)
		} else {
			let str = self.remaining_str();
			str[0..num_len].parse::<f32>().unwrap()
		};
		self.advance_n(num_len);

		match self.peek_byte(0) {
			b'%' => {
				self.pos += 1;
				Token::new_dimension(is_float, has_sign, num_len as u32, 1, value, atoms.str_to_bits("%") as u8)
			}
			_ => {
				let c = self.peek_char();
				let (_, c2, c3) = if c == EOF && self.at_end() { (EOF, EOF, EOF) } else { self.peek3() };
				if is_ident_start_sequence(c, c2, c3) {
					let (unit_len, _, _, _, atom_bits, _) = self.consume_ident_sequence(atoms);
					let atom = if atom_bits <= 127 { atom_bits as u8 } else { 0 };
					Token::new_dimension(is_float, has_sign, num_len as u32, unit_len, value, atom)
				} else {
					Token::new_number(is_float, has_sign, num_len as u32, value)
				}
			}
		}
	}

	fn consume_hash_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		self.pos += 1; // skip '#'
		let hash_str = self.remaining_str();
		let first_is_ascii = is_ident(self.peek_char());
		let hash_start = self.pos;
		let (len, contains_non_lower_ascii, _, contains_escape, _, _) = self.consume_ident_sequence(atoms);
		let hash_bytes = &self.bytes[hash_start..hash_start + len as usize];
		let mut hex_value: u32 = 0;
		let mut is_hex = false;
		if (len == 3 || len == 4) && !contains_escape {
			is_hex = true;
			for &b in hash_bytes.iter().take(len as usize) {
				if let Some(d) = (b as char).to_digit(16) {
					hex_value = (hex_value << 8) | (d << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		} else if (len == 3 || len == 4) && contains_escape {
			// Use char-based iteration for escaped sequences
			is_hex = true;
			for c in hash_str.chars().take(len as usize) {
				if let Some(d) = c.to_digit(16) {
					hex_value = (hex_value << 8) | (d << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		} else if (len == 6 || len == 8) && !contains_escape {
			is_hex = true;
			for &b in hash_bytes.iter().take(len as usize) {
				if let Some(d) = (b as char).to_digit(16) {
					hex_value = (hex_value << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		} else if (len == 6 || len == 8) && contains_escape {
			is_hex = true;
			for c in hash_str.chars().take(len as usize) {
				if let Some(d) = c.to_digit(16) {
					hex_value = (hex_value << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		}
		if is_hex && (len == 3 || len == 6) {
			hex_value = (hex_value << 8) | 0xFF;
		}
		if !is_hex {
			hex_value = 0;
		}
		Token::new_hash(contains_non_lower_ascii, first_is_ascii, contains_escape, len + 1, hex_value)
	}

	#[inline]
	fn consume_ident_like_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		let (mut len, contains_non_lower_ascii, dashed, contains_escape, atom_bits, is_url) =
			self.consume_ident_sequence(atoms);
		if self.peek_byte(0) == b'(' {
			self.pos += 1;
			len += 1;
			let token = Token::new_function(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len);
			if is_url {
				// Look ahead for up to 4 whitespace chars then check for quote
				let mut lookahead = 0usize;
				for _i in 0..=3 {
					let b = self.peek_byte(lookahead);
					if b < 128 && ASCII_WHITESPACE.0[b as usize] {
						lookahead += 1;
					} else {
						break;
					}
				}
				let next_b = self.peek_byte(lookahead);
				if !is_quote(next_b as char) || (next_b == 0 && self.pos + lookahead >= self.bytes.len()) {
					return self.consume_url_sequence(len, contains_escape);
				}
			}
			return token;
		}
		Token::new_ident(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len)
	}

	fn consume_string_token(&mut self) -> Token {
		let delimiter = self.advance_byte();
		let quotes = if delimiter == b'"' { QuoteStyle::Double } else { QuoteStyle::Single };
		let mut contains_escape = false;
		let mut len: u32 = 1;
		loop {
			let b = self.peek_byte(0);
			match b {
				// Check for newlines (LF=0x0A, CR=0x0D, FF=0x0C)
				b'\n' | b'\r' | 0x0C => {
					return Token::new_bad_string(len);
				}
				0 => {
					if self.at_end() {
						// EOF
						return Token::new_string(quotes, false, contains_escape, len);
					}
					// NUL byte in middle of string — treat as replacement char
					self.pos += 1;
					contains_escape = true;
					len += 1;
				}
				b'"' | b'\'' => {
					self.pos += 1;
					len += 1;
					if b == delimiter {
						return Token::new_string(quotes, true, contains_escape, len);
					}
				}
				b'\\' => {
					self.pos += 1;
					contains_escape = true;
					let next_b = self.peek_byte(0);
					if next_b == 0 && self.at_end() {
						// EOF after backslash
						len += 1;
						return Token::new_string(quotes, false, contains_escape, len);
					}
					// Check newline
					if next_b < 128 && ASCII_NEWLINE.0[next_b as usize] {
						len += self.consume_newline() + 1;
					} else {
						let next_c = self.peek_char();
						if is_escape_sequence('\\', next_c) {
							len += self.consume_escape_sequence();
						} else {
							return Token::new_bad_string(len);
						}
					}
				}
				_ => {
					// Bulk scan: find the next byte that needs per-byte handling.
					let remaining = self.remaining_bytes();
					let safe_end = scan_string(remaining, delimiter);
					if safe_end > 0 {
						self.advance_n(safe_end);
						len += safe_end as u32;
					} else {
						// First byte is itself a sentinel — handle one byte at a time
						if b < 128 {
							self.pos += 1;
							len += 1;
						} else {
							let c = self.peek_char();
							let clen = c.len_utf8();
							self.advance_n(clen);
							len += clen as u32;
						}
					}
				}
			}
		}
	}

	fn is_number_start(&self) -> bool {
		let remaining = self.remaining_bytes();
		if remaining.is_empty() {
			return false;
		}
		let first = remaining[0];
		if first.is_ascii_digit() {
			return true;
		}
		if (first == b'+' || first == b'-') && remaining.len() >= 2 {
			let second = remaining[1];
			return second.is_ascii_digit()
				|| (second == b'.' && remaining.len() >= 3 && remaining[2].is_ascii_digit());
		}
		first == b'.' && remaining.len() >= 2 && remaining[1].is_ascii_digit()
	}

	fn would_start_unicode_range(&self) -> bool {
		// Caller confirmed current byte is U or u. Check next two.
		let b1 = self.peek_byte(1);
		if b1 != b'+' {
			return false;
		}
		let b2 = self.peek_byte(2);
		b2 == b'?' || (b2 as char).is_ascii_hexdigit()
	}

	fn consume_unicode_range_token(&mut self) -> Token {
		// Step 1: Consume U/u and +
		self.pos += 1; // U or u
		self.pos += 1; // +
		let mut len: u32 = 2;

		// Step 2: Consume hex digits (up to 6)
		let mut hex_count: u32 = 0;
		let mut hex_value: u32 = 0;
		while hex_count < 6 {
			let b = self.peek_byte(0);
			if b < 128 {
				if let Some(d) = (b as char).to_digit(16) {
					self.pos += 1;
					hex_value = (hex_value << 4) | d;
					hex_count += 1;
					len += 1;
				} else {
					break;
				}
			} else {
				break;
			}
		}

		// Consume ? placeholders
		let mut question_count: u32 = 0;
		while hex_count + question_count < 6 && self.peek_byte(0) == b'?' {
			self.pos += 1;
			question_count += 1;
			len += 1;
		}

		if question_count > 0 {
			let start = hex_value << (question_count * 4);
			let end = start | ((1 << (question_count * 4)) - 1);
			return Token::new_unicode_range(start, end, len);
		}

		let start = hex_value;

		// Check for range: - followed by hex digit
		if self.peek_byte(0) == b'-' && (self.peek_byte(1) as char).is_ascii_hexdigit() {
			self.pos += 1; // -
			len += 1;

			let mut end_hex_count: u32 = 0;
			let mut end_value: u32 = 0;
			while end_hex_count < 6 {
				let b = self.peek_byte(0);
				if b < 128 {
					if let Some(d) = (b as char).to_digit(16) {
						self.pos += 1;
						end_value = (end_value << 4) | d;
						end_hex_count += 1;
						len += 1;
					} else {
						break;
					}
				} else {
					break;
				}
			}
			return Token::new_unicode_range(start, end_value, len);
		}

		Token::new_unicode_range(start, start, len)
	}
}

/// Fast integer parsing for CSS numeric tokens. Handles optional leading sign.
/// Only called when `is_float` is false (no decimal point or exponent).
/// For integers with 9 or fewer digits, this computes the value directly.
/// For larger integers, falls back to str::parse.
#[inline]
fn fast_parse_integer(bytes: &[u8], has_sign: bool, num_len: usize) -> f32 {
	let start = if has_sign { 1 } else { 0 };
	let digit_count = num_len - start;
	if digit_count <= 9 {
		let mut value: u32 = 0;
		let mut i = start;
		while i < num_len {
			value = value * 10 + (bytes[i] - b'0') as u32;
			i += 1;
		}
		let fvalue = value as f32;
		if has_sign && bytes[0] == b'-' { -fvalue } else { fvalue }
	} else {
		// Fallback for very large integers (> 9 digits)
		let str = unsafe { std::str::from_utf8_unchecked(&bytes[0..num_len]) };
		str.parse::<f32>().unwrap()
	}
}

/// Convert raw codepoint to char, handling 0 and surrogates -> REPLACEMENT_CHARACTER
#[inline]
fn codepoint_to_char(value: u32) -> char {
	use crate::syntax::SURROGATE_RANGE;
	if value == 0 || SURROGATE_RANGE.contains(&value) {
		REPLACEMENT_CHARACTER
	} else {
		char::from_u32(value).unwrap_or(REPLACEMENT_CHARACTER)
	}
}

impl<'a> Lexer<'a> {
	#[must_use]
	pub(crate) fn read_next_token(&mut self, offset: u32) -> Token {
		if self.source.len() as u32 == offset {
			return Token::EOF;
		}
		let bytes = self.source.as_bytes();
		let mut cursor = ByteCursor::new(&bytes[offset as usize..]);
		let b = cursor.peek_byte(0);
		// fast path for single character tokens (ASCII only)
		if b < 128 {
			let token = SINGLE_CHAR_TOKENS[b as usize];
			if token != Token::EOF {
				return token;
			}
			if b == b' ' && !is_whitespace(cursor.peek_byte(1) as char) {
				return Token::SPACE;
			}
			// fast path for identifiers
			if is_ident_ascii_start(b as char) {
				if matches!(b, b'U' | b'u')
					&& self.features.intersects(Feature::UnicodeRange)
					&& cursor.would_start_unicode_range()
				{
					return cursor.consume_unicode_range_token();
				}
				return cursor.consume_ident_like_token(self.atoms);
			}
		}
		let c = cursor.peek_char();
		match c {
			'\0' => {
				if !cursor.is_last() {
					let (_, c2, c3) = cursor.peek3();
					if is_ident_start_sequence(REPLACEMENT_CHARACTER, c2, c3) {
						return cursor.consume_ident_like_token(self.atoms);
					}
				}
				if cursor.next_char().is_some() { Token::REPLACEMENT_CHARACTER } else { Token::EOF }
			}
			c if is_whitespace(c) && !self.features.contains(Feature::SeparateWhitespace) => {
				let (len, style) = cursor.consume_whitespace();
				Token::new_whitespace(style, len)
			}
			// Whitespace Range
			TAB => Token::new_whitespace(Whitespace::Tab, cursor.consume_same(b'\t')),
			SPACE => Token::new_whitespace(Whitespace::Space, cursor.consume_same(b' ')),
			LF | CR | FF => {
				let mut len: u32 = 0;
				loop {
					let nb = cursor.peek_byte(0);
					if !matches!(nb, b'\n' | b'\r' | 0x0C) {
						break;
					}
					cursor.pos += 1;
					len += 1;
				}
				Token::new_whitespace(Whitespace::Newline, len)
			}
			// Quote Range
			c if is_quote(c) => cursor.consume_string_token(),
			// Digit Range
			c if c.is_ascii_digit() => cursor.consume_numeric_token(self.atoms),
			// Sign Range
			'-' => {
				if cursor.peek_byte(1) == b'-' && cursor.peek_byte(2) == b'>' {
					cursor.advance_n(3);
					return Token::CDC;
				}
				let c2 = cursor.peek_char_at(1);
				let c3 = cursor.peek_char_at(1 + c2.len_utf8());
				if is_ident_start_sequence(c, c2, c3) {
					return cursor.consume_ident_like_token(self.atoms);
				}
				if cursor.is_number_start() {
					return cursor.consume_numeric_token(self.atoms);
				}
				Token::DASH
			}
			// Dot or Plus
			'.' | '+' => {
				if cursor.is_number_start() {
					return cursor.consume_numeric_token(self.atoms);
				}
				Token::new_delim(c)
			}
			// Less Than
			'<' => {
				cursor.pos += 1;
				if cursor.peek_byte(0) == b'!' && cursor.peek_byte(1) == b'-' && cursor.peek_byte(2) == b'-' {
					cursor.advance_n(3);
					return Token::CDO;
				}
				Token::LESS_THAN
			}
			// Hash / Pound Sign
			'#' => {
				let c1 = cursor.peek_char_at(1);
				let c2 = cursor.peek_char_at(1 + c1.len_utf8());
				if is_ident(c1) || is_escape_sequence(c1, c2) {
					cursor.consume_hash_token(self.atoms)
				} else {
					Token::HASH
				}
			}
			// Commercial At
			'@' => {
				cursor.pos += 1;
				let (c1, c2, c3) = cursor.peek3();
				if is_ident_start_sequence(c1, c2, c3) {
					let (len, contains_non_lower_ascii, dashed, contains_escape, atom_bits, _) =
						cursor.consume_ident_sequence(self.atoms);
					return Token::new_atkeyword(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len + 1);
				}
				Token::AT
			}
			// Reverse Solidus
			'\\' => {
				let c2 = cursor.peek_char_at(1);
				if is_escape_sequence(c, c2) {
					return cursor.consume_ident_like_token(self.atoms);
				}
				Token::BACKSLASH
			}
			// Solidus
			'/' => {
				let b2 = cursor.peek_byte(1);
				match b2 {
					b'*' => {
						cursor.advance_n(2);
						let mut len: u32 = 2;
						let (c1, c2) = cursor.peek2();
						let comment_style = match c1 {
							'*' if c2 != '/' => CommentStyle::BlockStar,
							'#' => CommentStyle::BlockPound,
							'!' => CommentStyle::BlockBang,
							'-' | '=' => CommentStyle::BlockHeading,
							_ => CommentStyle::Block,
						};
						loop {
							let remaining = cursor.remaining_bytes();
							let offset = scan_byte(remaining, b'*');
							if offset == remaining.len() {
								len += remaining.len() as u32;
								cursor.advance_n(remaining.len());
								break;
							}
							let advance = offset + 1;
							cursor.advance_n(advance);
							len += advance as u32;
							if cursor.peek_byte(0) == b'/' {
								cursor.advance_n(1);
								len += 1;
								break;
							}
						}
						Token::new_comment(comment_style, len)
					}
					b'/' if self.features.intersects(Feature::SingleLineComments) => {
						cursor.advance_n(2);
						let mut len: u32 = 2;
						let comment_style = match cursor.peek_byte(0) {
							b'*' => CommentStyle::SingleStar,
							b'!' => CommentStyle::SingleBang,
							_ => CommentStyle::Single,
						};
						let remaining = cursor.remaining_bytes();
						let safe_end = scan_line_comment(remaining);
						len += safe_end as u32;
						cursor.advance_n(safe_end);
						Token::new_comment(comment_style, len)
					}
					_ => Token::SLASH,
				}
			}
			c if is_ident_start(c) => cursor.consume_ident_like_token(self.atoms),
			c => Token::new_delim(c),
		}
	}
}
