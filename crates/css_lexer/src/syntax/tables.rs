const T: bool = true;
const F: bool = false;

#[repr(C, align(64))]
pub struct Align64<T>(pub(crate) T);

/// Lookup table for ASCII identifier start characters (A-Z, a-z, _)
pub const ASCII_START: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, F, F, F, F, F,
]);

/// Lookup table for valid identifier bytes (A-Z, a-z, 0-9, -, _)
pub const BYTE_IS_IDENT: Align64<[bool; 256]> = Align64([
	// 0x00-0x7F: ASCII range
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, T, F, F, T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
	T, T, T, T, T, T, T, T, F, F, F, F, F,
	// 0x80-0xFF: Non-ASCII range (all false - need full Unicode handling)
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F,
]);

/// Lookup table for ASCII whitespace characters (SPACE, TAB, LF, CR, FF)
pub const ASCII_WHITESPACE: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, T, T, F, T, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F,
]);

/// Lookup table for ASCII newline characters (LF, CR, FF)
pub const ASCII_NEWLINE: Align64<[bool; 128]> = Align64([
	F, F, F, F, F, F, F, F, F, F, T, F, T, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F,
]);

/// Lookup table for ASCII non-printable characters
/// Includes control characters except TAB, LF, FF, CR
pub const ASCII_NON_PRINTABLE: Align64<[bool; 128]> = Align64([
	T, T, T, T, T, T, T, T, T, F, F, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
	F, F, F, F, F, F, F, F, F, F, F, F, F, T,
]);
