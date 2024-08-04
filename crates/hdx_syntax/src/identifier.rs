const T: bool = true;
const F: bool = false;

use super::is_escape_sequence;

#[repr(C, align(64))]
pub struct Align64<T>(pub(crate) T);

pub const ASCII_LOWER: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, F, F, F, F, F,
]);

pub const ASCII_START: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, F, F, F, F, F,
]);

pub const ASCII_CONTINUE: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, T, F, F, T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, F, F, F, F, F,
]);

#[inline]
pub fn is_ident_ascii_start(c: char) -> bool {
	ASCII_START.0[c as usize]
}

#[inline]
pub fn is_ident_ascii(c: char) -> bool {
	ASCII_CONTINUE.0[c as usize]
}

#[inline]
pub fn is_non_ascii(c: char) -> bool {
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

#[inline]
pub fn is_ident_start(c: char) -> bool {
	if c.is_ascii() {
		return is_ident_ascii_start(c);
	}
	is_non_ascii(c)
}

#[inline]
pub fn is_ident(c: char) -> bool {
	if c.is_ascii() {
		return is_ident_ascii(c);
	}
	is_non_ascii(c)
}

#[inline]
pub fn is_ident_ascii_lower(c: char) -> bool {
	c.is_ascii() && ASCII_LOWER.0[c as usize]
}

#[inline]
pub fn is_dash_or_ident_start(c: char) -> bool {
	c == '-' || is_ident_start(c)
}

#[inline]
pub fn is_ident_str(s: &str) -> bool {
	is_dash_or_ident_start(s.chars().next().unwrap()) && s.chars().all(is_ident)
}

#[inline]
pub fn is_ident_start_sequence(c: char, c2: char, c3: char) -> bool {
	if c == '-' {
		return c2 == '-' || is_ident_start(c2) || is_escape_sequence(c2, c3);
	}
	is_ident_start(c) || is_escape_sequence(c, c2)
}
