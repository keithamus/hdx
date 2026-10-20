use bytescan::{byte_class, scan_until, scan_until_and_mark};

byte_class! {
	/// Bytes that end an ASCII CSS identifier: any byte other than `a-z A-Z 0-9 - _`.
	struct IdentEnd = !(alpha | b'0'..=b'9' | b'-' | b'_');
}

byte_class! {
	/// ASCII uppercase letters, which force a case fold of an identifier.
	struct AsciiUpper = b'A'..=b'Z';
}

byte_class! {
	/// ASCII identifier bytes that are not lowercase letters, digits or `-`: `A-Z` and `_`.
	struct AsciiNonLowerIdent = b'A'..=b'Z' | b'_';
}

byte_class! {
	/// Bytes that end the body of a CSS string token: the delimiter, `\`, a newline (`\n` `\r`
	/// `\x0C`), NUL, or any non-ASCII byte.
	struct StringEnd(delimiter) = delimiter | b'\\' | b'\n' | b'\r' | 0x0C | 0 | 0x80..=0xFF;
}

byte_class! {
	/// Bytes that end the body of a CSS url token: whitespace or control bytes (`<= 0x20`), a
	/// quote, `(`, `)`, `\`, DEL (`0x7F`), or any non-ASCII byte.
	struct UrlEnd = 0x00..=0x20 | 0x80..=0xFF | b'\'' | b'"' | b'(' | b')' | b'\\' | 0x7F;
}

byte_class! {
	/// Bytes that end the remnants of a CSS bad-url token: `)` or `\`.
	struct BadUrlEnd = b')' | b'\\';
}

byte_class! {
	/// Bytes that end a CSS single-line comment: a newline (`\n` `\r` `\x0C`) or NUL.
	struct LineCommentEnd = b'\n' | b'\r' | 0x0C | 0;
}

#[inline]
pub(crate) fn scan_ident(bytes: &[u8]) -> (usize, bool) {
	scan_until_and_mark(bytes, IdentEnd, AsciiUpper)
}

#[inline]
pub(crate) fn scan_ident_non_lower(bytes: &[u8]) -> (usize, bool) {
	scan_until_and_mark(bytes, IdentEnd, AsciiNonLowerIdent)
}

#[inline]
pub(crate) fn scan_string(bytes: &[u8], delimiter: u8) -> usize {
	scan_until(bytes, StringEnd(delimiter))
}

#[inline]
pub(crate) fn scan_url(bytes: &[u8]) -> usize {
	scan_until(bytes, UrlEnd)
}

#[inline]
pub(crate) fn scan_bad_url(bytes: &[u8]) -> usize {
	scan_until(bytes, BadUrlEnd)
}

#[inline]
pub(crate) fn scan_line_comment(bytes: &[u8]) -> usize {
	scan_until(bytes, LineCommentEnd)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ident() {
		for (s, want) in [
			(&b""[..], (0, false)),
			(b"simple", (6, false)),
			(b"width:1px", (5, false)),
			(b"color) ", (5, false)),
			(b"has space", (3, false)),
			(b"Background-Color", (16, true)),
			(b"ALLCAPS", (7, true)),
			(b"CAFE\xc3\xa9", (4, true)),
			(b"emoji\xf0\x9f\x98\x80tail", (5, false)),
			(b"abcdefghijklmnoP", (16, true)),
			(b"abcdefghijklmno P", (15, false)),
			(b"abcdefghijklmnop;Q", (16, false)),
			(b"abcdefghijklmnopQRSTUVWXYZ0123456789", (36, true)),
			(b"abcdefghijklmnopqrstuvwxyz0123456;", (33, false)),
		] {
			assert_eq!(scan_ident(s), want, "ident {s:?}");
		}
	}

	#[test]
	fn ident_non_lower() {
		for (s, want) in [
			(&b""[..], (0, false)),
			(b"simple-1", (8, false)),
			(b"--dashed\\:x", (8, false)),
			(b"snake_case", (10, true)),
			(b"Upper", (5, true)),
			(b"a\\_b", (1, false)),
			(b"cafe\xc3\xa9_", (4, false)),
			(b"abcdefghijklmnop_", (17, true)),
			(b"abcdefghijklmnop:_", (16, false)),
			(b"abcdefghijklmnopqrstuvwxyz-0123456789_", (38, true)),
		] {
			assert_eq!(scan_ident_non_lower(s), want, "ident_non_lower {s:?}");
		}
	}

	#[test]
	fn string() {
		for (s, delim, want) in [
			(&b""[..], b'"', 0usize),
			(b"hello world", b'"', 11),
			(b"quote\"tail", b'"', 5),
			(b"back\\slash", b'"', 4),
			(b"line\nbreak", b'"', 4),
			(b"carriage\rreturn", b'"', 8),
			(b"formfeed\x0cbyte", b'"', 8),
			(b"nul\0byte", b'"', 3),
			(b"unicode\xc3\xa9tail", b'"', 7),
			(b"it's", b'\'', 2),
			(b"it's", b'"', 4),
			(b"0123456789abcde\"", b'"', 15),
			(b"0123456789abcdef\"g", b'"', 16),
			(b"0123456789abcdefghijk", b'"', 21),
		] {
			assert_eq!(scan_string(s, delim), want, "string {s:?} delim {delim:#x}");
		}
	}

	#[test]
	fn url() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"path/to/image.png", 17),
			(b"has space", 3),
			(b"paren)here", 5),
			(b"back\\slash", 4),
			(b"quote'x", 5),
			(b"open(x", 4),
			(b"del\x7fx", 3),
			(b"tab\tbyte", 3),
			(b"unicode\xe2\x82\xactail", 7),
			(b"0123456789abcde)", 15),
			(b"0123456789abcdef)", 16),
		] {
			assert_eq!(scan_url(s), want, "url {s:?}");
		}
	}

	#[test]
	fn bad_url() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"cleantail", 9),
			(b"close)here", 5),
			(b"back\\slash", 4),
			(b"0123456789abcde)", 15),
			(b"0123456789abcdef)", 16),
		] {
			assert_eq!(scan_bad_url(s), want, "bad_url {s:?}");
		}
	}

	#[test]
	fn line_comment() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"comment body", 12),
			(b"line\nfeed", 4),
			(b"carriage\rreturn", 8),
			(b"formfeed\x0cbyte", 8),
			(b"nul\0byte", 3),
			(b"0123456789abcde\n", 15),
			(b"0123456789abcdef\n", 16),
		] {
			assert_eq!(scan_line_comment(s), want, "line_comment {s:?}");
		}
	}
}
