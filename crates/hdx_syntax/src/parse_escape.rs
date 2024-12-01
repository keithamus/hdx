use std::{char::REPLACEMENT_CHARACTER, str::Chars};

use crate::{is_whitespace, SURROGATE_RANGE};

pub trait ParseEscape {
	fn parse_escape_sequence(&mut self) -> (char, u8);
}

impl<'a> ParseEscape for Chars<'a> {
	fn parse_escape_sequence(&mut self) -> (char, u8) {
		if let Some(c) = self.next() {
			if !c.is_ascii_hexdigit() {
				return (c, c.len_utf8() as u8);
			}
			let mut value = 0;
			let mut i = 0;
			let mut c = c;
			while let Some(v) = c.to_digit(16) {
				value = (value << 4) | v;
				i += 1;
				c = self.next().unwrap_or(REPLACEMENT_CHARACTER);
				if i > 5 {
					break;
				}
			}
			if is_whitespace(c) {
				i += 1;
				if c == '\r' && self.next() == Some('\n') {
					i += 1;
				}
			}
			if value == 0 || SURROGATE_RANGE.contains(&value) {
				return (REPLACEMENT_CHARACTER, i);
			}
			(char::from_u32(value).unwrap_or(REPLACEMENT_CHARACTER), i)
		} else {
			(REPLACEMENT_CHARACTER, 0)
		}
	}
}
