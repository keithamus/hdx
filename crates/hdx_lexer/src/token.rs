use std::hash::{Hash, Hasher};

use bitmask_enum::bitmask;
use hdx_atom::Atom;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Default)]
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum NumType {
	#[default]
	Float = 0x01,
	Signed = 0x10,
}

impl NumType {
	#[inline]
	pub fn is_int(&self) -> bool {
		self.and(NumType::Float) != NumType::Float
	}

	#[inline]
	pub fn is_float(&self) -> bool {
		self.contains(NumType::Float)
	}

	#[inline]
	pub fn is_signed(&self) -> bool {
		self.contains(NumType::Signed)
	}

	#[inline]
	pub fn is_signed_int(&self) -> bool {
		self.contains(NumType::Signed) && !self.contains(NumType::Float)
	}

	#[inline]
	pub fn signed(&self) -> NumType {
		self.or(NumType::Signed)
	}

	#[inline]
	pub fn float(&self) -> NumType {
		self.or(NumType::Float)
	}
}

#[derive(Default)]
#[bitmask(u8)] // Actually more like a "u5" as the 3 LMB are unused
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

	// Variable length Atom containing Tokens (mask: 0b0_1XXXX)
	Ident = 0b1000,     // https://drafts.csswg.org/css-syntax/#ident-token-diagram
	Function = 0b1001,  // https://drafts.csswg.org/css-syntax/#function-token-diagram
	AtKeyword = 0b1010, // https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	Hash = 0b1011,      // https://drafts.csswg.org/css-syntax/#hash-token-diagram
	HashId = 0b1100,    // https://drafts.csswg.org/css-syntax/#hash-token-diagram
	String = 0b1101,    // https://drafts.csswg.org/css-syntax/#string-token-diagram
	Url = 0b1110,       // https://drafts.csswg.org/css-syntax/#url-token-diagram

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
	pub fn as_str(&self) -> &str {
		match *self {
			Kind::Eof => "EOF",
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
			Kind::HashId => "HashId",
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
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum Token {
	// <eof-token> - the end of a file (https://drafts.csswg.org/css-syntax/#typedef-eof-token)
	Eof,

	// <comment-token> (https://drafts.csswg.org/css-syntax/#comment-diagram)
	Comment(Atom),

	// <ident-token> (https://drafts.csswg.org/css-syntax/#ident-token-diagram)
	Ident(Atom),

	// <function-token> (https://drafts.csswg.org/css-syntax/#function-token-diagram)
	Function(Atom),

	// <at-keyword> https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	AtKeyword(Atom),

	// "unrestricted" <hash-token> (https://drafts.csswg.org/css-syntax/#hash-token-diagram)
	Hash(Atom),

	// "id" <hash-token> (https://drafts.csswg.org/css-syntax/#hash-token-diagram)
	HashId(Atom),

	// <string-token> (https://drafts.csswg.org/css-syntax/#string-token-diagram)
	String(Atom, QuoteStyle),

	// <bad-string-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-string-token)
	BadString,

	// <url-token> (https://drafts.csswg.org/css-syntax/#url-token-diagram)
	Url(Atom, QuoteStyle),

	// <bad-url-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-url-token)
	BadUrl,

	// <delim-token> (https://drafts.csswg.org/css-syntax/#typedef-delim-token)
	Delim(char),

	// <number-token> (https://drafts.csswg.org/css-syntax/#number-token-diagram)
	Number(f32, NumType),

	// <dimension-token> (https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
	Dimension(f32, Atom, NumType),

	#[default]
	// <whitespace-token> (https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
	Whitespace,

	// <cdo-token> (https://drafts.csswg.org/css-syntax/#CDO-token-diagram)
	Cdo,

	// <cdc-token> (https://drafts.csswg.org/css-syntax/#CDC-token-diagram)
	Cdc,

	// <colon-token> (https://drafts.csswg.org/css-syntax/#typedef-colon-token)
	Colon,

	// <semicolon-token> (https://drafts.csswg.org/css-syntax/#typedef-semicolon-token)
	Semicolon,

	// <comma-token> (https://drafts.csswg.org/css-syntax/#typedef-comma-token)
	Comma,

	// <[-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-square)
	LeftSquare,

	// <]-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-square)
	RightSquare,

	// <(-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-paren)
	LeftParen,

	// <)-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-paren)
	RightParen,

	// <{-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-curly)
	LeftCurly,

	// <}-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-curly)
	RightCurly,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	// Some tokens/ast nodesthat would otherwise be strings (e.g. url(), named fonts) can have no quotes.
	None,
	Single,
	#[default]
	Double,
}

impl Token {
	#[inline(always)]
	pub(crate) fn kind_bits(&self) -> u8 {
		self.kind().bits
	}

	#[inline]
	pub fn kind(&self) -> Kind {
        match self {
            Self::Eof => Kind::Eof,
            Self::Comment(_) => Kind::Comment,
            Self::Ident(_) => Kind::Ident,
            Self::Function(_) => Kind::Function,
            Self::AtKeyword(_) => Kind::AtKeyword,
            Self::Hash(_) => Kind::Hash,
            Self::HashId(_) => Kind::HashId,
            Self::String(_, _) => Kind::String,
            Self::BadString => Kind::BadString,
            Self::Url(_, _) => Kind::Url,
            Self::BadUrl => Kind::BadUrl,
            Self::Delim(_) => Kind::Delim,
            Self::Number(_, _) => Kind::Number,
            Self::Dimension(_, _, _) => Kind::Dimension,
            Self::Whitespace => Kind::Whitespace,
            Self::Cdo => Kind::CdcOrCdo,
            Self::Cdc => Kind::CdcOrCdo,
            Self::Colon => Kind::Colon,
            Self::Semicolon => Kind::Semicolon,
            Self::Comma => Kind::Comma,
            Self::LeftSquare => Kind::LeftSquare,
            Self::RightSquare => Kind::RightSquare,
            Self::LeftParen => Kind::LeftParen,
            Self::RightParen => Kind::RightParen,
            Self::LeftCurly => Kind::LeftCurly,
            Self::RightCurly => Kind::RightCurly,
        }
	}

	#[inline(always)]
	fn is_ident_like(&self) -> bool {
		self.kind_bits() & 0b11000 == 0b01000 && self.kind_bits() != Kind::String.bits
	}

	#[inline]
	pub fn has_atom(&self) -> bool {
		self.kind_bits() | 0b0_0111 == 0b0_1111
	}

	#[inline]
	pub fn is_character_token(&self) -> bool {
		self.kind_bits() | 0b0_1111 == 0b1_1111
	}

	pub fn is_dashed_ident(&self) -> bool {
		match self {
			Token::Ident(value) => value.starts_with("--"),
			_ => false,
		}
	}

	pub fn char(&self) -> Option<char> {
		match self {
            Self::Delim(c) => Some(*c),
            Self::Colon => Some(':'),
            Self::Semicolon => Some(';'),
            Self::Comma => Some(','),
            Self::LeftSquare => Some('['),
            Self::RightSquare => Some(']'),
            Self::LeftParen => Some('('),
            Self::RightParen => Some(')'),
            Self::LeftCurly => Some('{'),
            Self::RightCurly => Some('}'),
            _ => None
		}
	}

	#[inline]
	pub fn is_trivia(&self) -> bool {
		self.kind_bits() & 0b000011 == self.kind_bits()
	}

	#[inline]
	pub fn is_bad(&self) -> bool {
		(self.kind_bits() | 0b00001) & 0b11001 == 1
	}

	#[inline]
	pub fn is_numeric(&self) -> bool {
		self.kind_bits() | 0b0_0001 == 0b0_0101
	}


	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PairWise {
	Paren,
	Curly,
	Square,
}

impl PairWise {
	pub fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::LeftParen | Token::Function(_) => Some(Self::Paren),
			Token::LeftCurly => Some(Self::Curly),
			Token::LeftSquare => Some(Self::Square),
			Token::RightParen => Some(Self::Paren),
			Token::RightCurly => Some(Self::Curly),
			Token::RightSquare => Some(Self::Square),
			_ => None,
		}
	}

	pub fn start(&self) -> &Token {
		match self {
			Self::Paren => &Token::LeftParen,
			Self::Curly => &Token::LeftCurly,
			Self::Square => &Token::LeftSquare,
		}
	}

	pub fn end(&self) -> &Token {
		match self {
			Self::Paren => &Token::RightParen,
			Self::Curly => &Token::RightCurly,
			Self::Square => &Token::RightSquare,
		}
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Hash for Token {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Token::Eof => 0.hash(state),
			Token::Comment(a) => {
				1.hash(state);
				a.hash(state);
			}
			Token::Ident(a) => {
				2.hash(state);
				a.hash(state);
			}
			Token::Function(a) => {
				3.hash(state);
				a.hash(state);
			}
			Token::AtKeyword(a) => {
				4.hash(state);
				a.hash(state);
			}
			Token::Hash(a) => {
				5.hash(state);
				a.hash(state);
			}
			Token::HashId(a) => {
				6.hash(state);
				a.hash(state);
			}
			Token::String(a, s) => {
				7.hash(state);
				a.hash(state);
				s.hash(state);
			}
			Token::BadString => {
				8.hash(state);
			}
			Token::Url(a, s) => {
				9.hash(state);
				a.hash(state);
				s.hash(state);
			}
			Token::BadUrl => {
				10.hash(state);
			}
			Token::Delim(c) => {
				11.hash(state);
				c.hash(state);
			}
			Token::Number(f, n) => {
				12.hash(state);
				f.to_bits().hash(state);
				n.hash(state);
			}
			Token::Dimension(f, a, n) => {
				13.hash(state);
				f.to_bits().hash(state);
				a.hash(state);
				n.hash(state);
			}
			Token::Whitespace => 14.hash(state),
			Token::Cdo => 15.hash(state),
			Token::Cdc => 16.hash(state),
			Token::Colon => 17.hash(state),
			Token::Semicolon => 18.hash(state),
			Token::Comma => 19.hash(state),
			Token::LeftSquare => 20.hash(state),
			Token::RightSquare => 21.hash(state),
			Token::LeftParen => 22.hash(state),
			Token::RightParen => 23.hash(state),
			Token::LeftCurly => 24.hash(state),
			Token::RightCurly => 25.hash(state),
		}
	}
}
