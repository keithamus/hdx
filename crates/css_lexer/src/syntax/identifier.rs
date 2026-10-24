use super::{
	is_escape_sequence,
	tables::{ASCII_START, BYTE_IS_IDENT},
};

#[inline(always)]
pub const fn is_ident_ascii_start(c: char) -> bool {
	ASCII_START.0[c as usize]
}

#[inline(always)]
pub const fn is_non_ascii(c: char) -> bool {
	if c as usize >= 0x10000 {
		return true;
	}
	matches!(c,
		'\u{00b7}' | '\u{200c}' | '\u{200d}' | '\u{203f}' | '\u{2040}' |
		'\u{00c0}'..='\u{00d6}' | '\u{00d8}'..='\u{00f6}' |
		'\u{00f8}'..='\u{037d}' | '\u{037f}'..='\u{1fff}' |
		'\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}' |
		'\u{3001}'..='\u{d7ff}' | '\u{f900}'..='\u{fdcf}' |
		'\u{fdf0}'..='\u{fffd}'
	)
}

#[inline(always)]
pub fn is_ident_start(c: char) -> bool {
	if c.is_ascii() {
		return is_ident_ascii_start(c);
	}
	is_non_ascii(c)
}

#[inline(always)]
pub fn is_ident(c: char) -> bool {
	if c.is_ascii() {
		return is_ident_byte(c as u8);
	}
	is_non_ascii(c)
}

#[inline(always)]
pub const fn is_ident_byte(byte: u8) -> bool {
	BYTE_IS_IDENT.0[byte as usize]
}

#[inline(always)]
pub fn is_ident_start_sequence(c: char, c2: char, c3: char) -> bool {
	if c == '-' {
		return c2 == '-' || is_ident_start(c2) || is_escape_sequence(c2, c3);
	}
	is_ident_start(c) || is_escape_sequence(c, c2)
}
