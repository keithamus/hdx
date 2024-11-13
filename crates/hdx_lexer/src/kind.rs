use core::fmt;

use crate::KindSet;

/// Kind represents the token "Type", categorised mostly by the token types
/// within the CSS Syntax spec. Maintaining parity with the spec makes it
/// easier to reason about logica round the parser, despite it being possible
/// to group a bunch of these tokens into a single "delimiter" token.
///
/// Importantly, `Kind` is represented as `u8` and must only use the 5 low bits,
/// because the upper 3 bits get used to house details about each kind, that a
/// token would be interested in learning about.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
	// Trivias (mask as 0b0_00XX)
	Eof = 0b0000, // https://drafts.csswg.org/css-syntax/#typedef-eof-token
	#[default]
	Whitespace = 0b0001, // https://drafts.csswg.org/css-syntax/#whitespace-token-diagram
	Comment = 0b0010, // https://drafts.csswg.org/css-syntax/#comment-diagram
	// Stand in for both the CDC and CDO tokens
	CdcOrCdo = 0b0011, // https://drafts.csswg.org/css-syntax/#CDO-token-diagram, https://drafts.csswg.org/css-syntax/#CDC-token-diagram

	// Numerics (mask as 0b0_010X)
	Number = 0b0100,    // https://drafts.csswg.org/css-syntax/#number-token-diagram
	Dimension = 0b0101, // https://drafts.csswg.org/css-syntax/#dimension-token-diagram

	// Errors (mask as 0b0_011X)
	BadString = 0b0110, // https://drafts.csswg.org/css-syntax/#typedef-bad-string-token
	BadUrl = 0b0111,    // https://drafts.csswg.org/css-syntax/#typedef-bad-url-token

	// Variable length Atom containing Tokens (mask: 0b0_1XXX)
	Ident = 0b1000,     // https://drafts.csswg.org/css-syntax/#ident-token-diagram
	Function = 0b1001,  // https://drafts.csswg.org/css-syntax/#function-token-diagram
	AtKeyword = 0b1010, // https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	Hash = 0b1011,      // https://drafts.csswg.org/css-syntax/#hash-token-diagram
	String = 0b1100,    // https://drafts.csswg.org/css-syntax/#string-token-diagram
	Url = 0b1101,       // https://drafts.csswg.org/css-syntax/#url-token-diagram

	// Single character Tokens (mask 0b1_XXXX)
	Delim = 0b1_0000,       // https://drafts.csswg.org/css-syntax/#typedef-delim-token
	Colon = 0b1_0001,       // https://drafts.csswg.org/css-syntax/#typedef-colon-token
	Semicolon = 0b1_0010,   // https://drafts.csswg.org/css-syntax/#typedef-semicolon-token
	Comma = 0b1_0011,       // https://drafts.csswg.org/css-syntax/#typedef-comma-token
	LeftSquare = 0b1_0100,  // https://drafts.csswg.org/css-syntax/#tokendef-open-square
	RightSquare = 0b1_0101, // https://drafts.csswg.org/css-syntax/#tokendef-close-square
	LeftParen = 0b1_0110,   // https://drafts.csswg.org/css-syntax/#tokendef-open-paren
	RightParen = 0b1_0111,  // https://drafts.csswg.org/css-syntax/#tokendef-close-paren
	LeftCurly = 0b1_1000,   // https://drafts.csswg.org/css-syntax/#tokendef-open-curly
	RightCurly = 0b1_1001,  // https://drafts.csswg.org/css-syntax/#tokendef-close-curly
}

impl Kind {
	pub(crate) const fn from_bits(bits: u8) -> Self {
		match bits {
			0b0001 => Self::Whitespace,
			0b0010 => Self::Comment,
			0b0011 => Self::CdcOrCdo,
			0b0100 => Self::Number,
			0b0101 => Self::Dimension,
			0b0110 => Self::BadString,
			0b0111 => Self::BadUrl,
			0b1000 => Self::Ident,
			0b1001 => Self::Function,
			0b1010 => Self::AtKeyword,
			0b1011 => Self::Hash,
			0b1100 => Self::String,
			0b1101 => Self::Url,
			0b1_0000 => Self::Delim,
			0b1_0001 => Self::Colon,
			0b1_0010 => Self::Semicolon,
			0b1_0011 => Self::Comma,
			0b1_0100 => Self::LeftSquare,
			0b1_0101 => Self::RightSquare,
			0b1_0110 => Self::LeftParen,
			0b1_0111 => Self::RightParen,
			0b1_1000 => Self::LeftCurly,
			0b1_1001 => Self::RightCurly,
			_ => Self::Eof,
		}
	}

	pub const fn as_str(&self) -> &str {
		match *self {
			Kind::Eof => "Eof",
			Kind::Whitespace => "Whitespace",
			Kind::Comment => "Comment",
			Kind::CdcOrCdo => "CdcOrCdo",
			Kind::Number => "Number",
			Kind::Dimension => "Dimension",
			Kind::BadString => "BadString",
			Kind::BadUrl => "BadUrl",
			Kind::Ident => "Ident",
			Kind::Function => "Function",
			Kind::AtKeyword => "AtKeyword",
			Kind::Hash => "Hash",
			Kind::String => "String",
			Kind::Url => "Url",
			Kind::Delim => "Delim",
			Kind::Colon => "Colon",
			Kind::Semicolon => "Semicolon",
			Kind::Comma => "Comma",
			Kind::LeftSquare => "LeftSquare",
			Kind::RightSquare => "RightSquare",
			Kind::LeftParen => "LeftParen",
			Kind::RightParen => "RightParen",
			Kind::LeftCurly => "LeftCurly",
			Kind::RightCurly => "RightCurly",
		}
	}

	pub const fn ambiguous_without_whitespace(&self) -> bool {
		matches!(self, Kind::AtKeyword | Kind::Number | Kind::Hash | Kind::Ident | Kind::Dimension)
	}
}

impl fmt::Debug for Kind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Kind::{}", self.as_str())
	}
}

impl fmt::Display for Kind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Kind::{}", self.as_str())
	}
}

impl PartialEq<KindSet> for Kind {
	fn eq(&self, other: &KindSet) -> bool {
		other.contains_bits(*self as u8)
	}
}

#[test]
fn test_from_bits() {
	assert_eq!(Kind::from_bits(Kind::Eof as u8), Kind::Eof);
	assert_eq!(Kind::from_bits(Kind::Whitespace as u8), Kind::Whitespace);
	assert_eq!(Kind::from_bits(Kind::Comment as u8), Kind::Comment);
	assert_eq!(Kind::from_bits(Kind::CdcOrCdo as u8), Kind::CdcOrCdo);
	assert_eq!(Kind::from_bits(Kind::Number as u8), Kind::Number);
	assert_eq!(Kind::from_bits(Kind::Dimension as u8), Kind::Dimension);
	assert_eq!(Kind::from_bits(Kind::BadString as u8), Kind::BadString);
	assert_eq!(Kind::from_bits(Kind::BadUrl as u8), Kind::BadUrl);
	assert_eq!(Kind::from_bits(Kind::Ident as u8), Kind::Ident);
	assert_eq!(Kind::from_bits(Kind::Function as u8), Kind::Function);
	assert_eq!(Kind::from_bits(Kind::AtKeyword as u8), Kind::AtKeyword);
	assert_eq!(Kind::from_bits(Kind::Hash as u8), Kind::Hash);
	assert_eq!(Kind::from_bits(Kind::String as u8), Kind::String);
	assert_eq!(Kind::from_bits(Kind::Url as u8), Kind::Url);
	assert_eq!(Kind::from_bits(Kind::Delim as u8), Kind::Delim);
	assert_eq!(Kind::from_bits(Kind::Colon as u8), Kind::Colon);
	assert_eq!(Kind::from_bits(Kind::Semicolon as u8), Kind::Semicolon);
	assert_eq!(Kind::from_bits(Kind::Comma as u8), Kind::Comma);
	assert_eq!(Kind::from_bits(Kind::LeftSquare as u8), Kind::LeftSquare);
	assert_eq!(Kind::from_bits(Kind::RightSquare as u8), Kind::RightSquare);
	assert_eq!(Kind::from_bits(Kind::LeftParen as u8), Kind::LeftParen);
	assert_eq!(Kind::from_bits(Kind::RightParen as u8), Kind::RightParen);
	assert_eq!(Kind::from_bits(Kind::LeftCurly as u8), Kind::LeftCurly);
	assert_eq!(Kind::from_bits(Kind::RightCurly as u8), Kind::RightCurly);
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Kind>(), 1);
}
