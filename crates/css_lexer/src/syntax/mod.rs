pub mod identifier;
pub mod parse_escape;
pub mod url;

pub use parse_escape::*;

pub const SURROGATE_RANGE: std::ops::RangeInclusive<u32> = 0xd800..=0xdfff;

pub const EOF: char = '\0';

pub const FF: char = '\u{c}';

pub const CR: char = '\u{d}';

pub const LF: char = '\u{a}';

pub const TAB: char = '\u{9}';

pub const SPACE: char = ' ';

#[inline(always)]
pub fn is_whitespace(c: char) -> bool {
	c == SPACE || c == TAB || is_newline(c)
}

#[inline(always)]
pub fn is_newline(c: char) -> bool {
	c == CR || c == LF || c == FF
}

#[inline(always)]
pub fn is_sign(c: char) -> bool {
	c == '+' || c == '-'
}

#[inline(always)]
pub fn is_quote(c: char) -> bool {
	c == '\'' || c == '"'
}

#[inline(always)]
pub fn is_escape_sequence(c: char, c2: char) -> bool {
	c == '\\' && !is_newline(c2)
}

#[cfg(test)]
mod tests {
	use super::{identifier::is_ident_start_sequence, CR, FF, LF};

	#[test]
	fn test_is_ident_start_sequence() {
		assert!(is_ident_start_sequence('-', '-', 'a'));
		assert!(!is_ident_start_sequence('\0', '\0', '\0'));
		assert!(is_ident_start_sequence('\u{FFFD}', '\u{FFFD}', '\u{FFFD}'));
		assert!(!is_ident_start_sequence(CR, LF, FF));
	}
}
