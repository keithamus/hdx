use std::hash::{Hash, Hasher};

use hdx_atom::Atom;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum NumType {
	#[default]
	UnsignedInt,
	SignedInt,
	UnsignedFloat,
	SignedFloat,
}

impl NumType {
	pub fn is_int(&self) -> bool {
		matches!(*self, NumType::UnsignedInt | NumType::SignedInt)
	}

	pub fn signed(&self) -> NumType {
		match *self {
			NumType::UnsignedInt => NumType::SignedInt,
			NumType::UnsignedFloat => NumType::SignedFloat,
			x => x,
		}
	}

	pub fn float(&self) -> NumType {
		match *self {
			NumType::UnsignedInt => NumType::UnsignedFloat,
			NumType::SignedInt => NumType::SignedFloat,
			x => x,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum Token {
	#[default]
	// A token yet to be built
	Undetermined,

	// <eof-token> - the end of a file (https://drafts.csswg.org/css-syntax/#typedef-eof-token)
	Eof,

	// <comment-token> (https://drafts.csswg.org/css-syntax/#comment-diagram)
	Comment,

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
	String(Atom),

	// <bad-string-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-string-token)
	BadString,

	// <url-token> (https://drafts.csswg.org/css-syntax/#url-token-diagram)
	Url(Atom),

	// <bad-url-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-url-token)
	BadUrl,

	// <delim-token> (https://drafts.csswg.org/css-syntax/#typedef-delim-token)
	Delim(char),

	// <number-token> (https://drafts.csswg.org/css-syntax/#number-token-diagram)
	Number(NumType, f32),

	// <dimension-token> (https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
	Dimension(NumType, f32, Atom),

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

impl Token {
	#[inline]
	pub fn constains_escape(&self) -> bool {
		match *self {
			Token::Ident(_)
			| Token::Function(_)
			| Token::AtKeyword(_)
			| Token::Hash(_)
			| Token::HashId(_)
			| Token::String(_)
			| Token::BadString
			| Token::Url(_) => true,
			_ => false,
		}
	}

	#[inline]
	pub fn is_trivia(&self) -> bool {
		matches!(self, Token::Whitespace | Token::Comment)
	}

	#[inline]
	pub fn is_bad(&self) -> bool {
		matches!(self, Token::BadString | Token::BadUrl)
	}

	#[inline]
	pub fn as_atom(&self) -> Option<Atom> {
		match self {
			Token::Ident(value)
			| Token::Function(value)
			| Token::AtKeyword(value)
			| Token::Hash(value)
			| Token::HashId(value)
			| Token::String(value)
			| Token::Url(value) => Some(value.clone()),
			_ => None,
		}
	}

	#[inline]
	pub fn as_atom_lower(&self) -> Option<Atom> {
		self.as_atom().map(|s| s.to_ascii_lowercase())
	}

	pub fn matches_ignore_case(&self, str: &Atom) -> bool {
		self.as_atom().map_or(false, |s| s.eq_ignore_ascii_case(str))
	}

	pub fn is_function_like(&self) -> bool {
		matches!(self, Token::Url(_) | Token::Function(_))
	}

	pub fn is_dashed_ident(&self) -> bool {
		match self {
			Token::Ident(value) => value.starts_with("--"),
			_ => false,
		}
	}

	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}

	pub fn as_f32(&self) -> Option<f32> {
		match self {
			Self::Number(_, value) => Some(*value),
			Self::Dimension(_, value, _) => Some(*value),
			_ => None,
		}
	}

	pub fn as_i32(&self) -> Option<i32> {
		match self {
			Self::Number(_, value) => Some(*value as i32),
			Self::Dimension(_, value, _) => Some(*value as i32),
			_ => None,
		}
	}

	pub fn as_char(&self) -> Option<char> {
		match self {
			Self::Delim(s) => Some(*s),
			_ => None,
		}
	}

	pub fn is_signed(&self) -> bool {
		match self {
			Self::Number(NumType::SignedInt, _) => true,
			Self::Number(NumType::SignedFloat, _) => true,
			Self::Dimension(NumType::SignedInt, _, _) => true,
			Self::Dimension(NumType::SignedFloat, _, _) => true,
			_ => false,
		}
	}

	pub fn is_int(&self) -> bool {
		match self {
			Self::Number(NumType::SignedInt, _) => true,
			Self::Number(NumType::UnsignedInt, _) => true,
			Self::Dimension(NumType::SignedInt, _, _) => true,
			Self::Dimension(NumType::UnsignedInt, _, _) => true,
			_ => false,
		}
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

	pub fn start(&self) -> Token {
		match self {
			Self::Paren => Token::LeftParen,
			Self::Curly => Token::LeftCurly,
			Self::Square => Token::LeftSquare,
		}
	}

	pub fn end(&self) -> Token {
		match self {
			Self::Paren => Token::RightParen,
			Self::Curly => Token::RightCurly,
			Self::Square => Token::RightSquare,
		}
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl Hash for Token {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Token::Undetermined => {}
			Token::Eof => 0.hash(state),
			Token::Comment => 1.hash(state),
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
			Token::String(a) => {
				7.hash(state);
				a.hash(state);
			}
			Token::BadString => {
				8.hash(state);
			}
			Token::Url(a) => {
				9.hash(state);
				a.hash(state);
			}
			Token::BadUrl => {
				10.hash(state);
			}
			Token::Delim(c) => {
				11.hash(state);
				c.hash(state);
			}
			Token::Number(n, f) => {
				12.hash(state);
				n.hash(state);
				f.to_bits().hash(state);
			}
			Token::Dimension(n, f, a) => {
				13.hash(state);
				n.hash(state);
				f.to_bits().hash(state);
				a.hash(state);
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
