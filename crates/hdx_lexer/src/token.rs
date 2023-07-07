use std::hash::{Hash, Hasher};

use hdx_atom::Atom;
#[cfg(feature = "serde")]
use serde::Serialize;

use super::kind::Kind;
use crate::Span;

#[derive(Debug, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Token {
	pub kind: Kind,

	#[cfg_attr(feature = "serde", serde(flatten))]
	pub span: Span,

	pub escaped: bool,

	pub value: TokenValue,
}

impl Token {
	#[inline]
	pub fn is_trivia(&self) -> bool {
		self.kind.is_trivia()
	}

	#[inline]
	pub fn is_bad(&self) -> bool {
		self.kind.is_bad()
	}

	#[inline]
	pub fn as_atom(&self) -> Option<Atom> {
		self.value.as_atom()
	}

	#[inline]
	pub fn as_atom_lower(&self) -> Option<Atom> {
		self.value.as_atom_lower()
	}

	pub fn matches_ignore_case(&self, str: &Atom) -> bool {
		self.value.as_atom().map_or(false, |s| s.eq_ignore_ascii_case(str))
	}

	pub fn is_dashed_ident(&self) -> bool {
		match self.kind {
			Kind::Ident => self.value.as_atom().unwrap().starts_with("--"),
			_ => false,
		}
	}

	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
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
		match token.kind {
			Kind::LeftParen | Kind::Function => Some(Self::Paren),
			Kind::LeftCurly => Some(Self::Curly),
			Kind::LeftSquare => Some(Self::Square),
			Kind::RightParen => Some(Self::Paren),
			Kind::RightCurly => Some(Self::Curly),
			Kind::RightSquare => Some(Self::Square),
			_ => None,
		}
	}

	pub fn start(&self) -> Kind {
		match self {
			Self::Paren => Kind::LeftParen,
			Self::Curly => Kind::LeftCurly,
			Self::Square => Kind::LeftSquare,
		}
	}

	pub fn end(&self) -> Kind {
		match self {
			Self::Paren => Kind::RightParen,
			Self::Curly => Kind::RightCurly,
			Self::Square => Kind::RightSquare,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum TokenValue {
	None,
	String(Atom),
	Char(char),
	Number { value: f32, signed: bool, int: bool },
	Dimension { value: f32, signed: bool, int: bool, unit: Atom },
	Unrestricted(Atom),
}

impl Default for TokenValue {
	fn default() -> Self {
		Self::None
	}
}

impl TokenValue {
	pub fn as_f32(&self) -> Option<f32> {
		match self {
			Self::Number { value, .. } => Some(*value),
			Self::Dimension { value, .. } => Some(*value),
			_ => None,
		}
	}

	pub fn as_i32(&self) -> Option<i32> {
		match self {
			Self::Number { value, .. } => Some(*value as i32),
			Self::Dimension { value, .. } => Some(*value as i32),
			_ => None,
		}
	}

	pub fn as_atom(&self) -> Option<Atom> {
		match self {
			Self::String(s) => Some(s.into()),
			Self::Unrestricted(s) => Some(s.into()),
			Self::Dimension { unit, .. } => Some(unit.into()),
			_ => None,
		}
	}

	pub fn as_atom_lower(&self) -> Option<Atom> {
		self.as_atom().map(|s| s.to_ascii_lowercase())
	}

	pub fn as_char(&self) -> Option<char> {
		match self {
			Self::Char(s) => Some(*s),
			_ => None,
		}
	}

	pub fn is_signed(&self) -> bool {
		match self {
			Self::Number { signed, .. } => *signed,
			Self::Dimension { signed, .. } => *signed,
			_ => false,
		}
	}

	pub fn is_int(&self) -> bool {
		match self {
			Self::Number { int, .. } => *int,
			Self::Dimension { int, .. } => *int,
			_ => false,
		}
	}
}

impl Hash for TokenValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::None => 0.hash(state),
			Self::String(s) => {
				1.hash(state);
				s.hash(state);
			}
			Self::Char(c) => {
				2.hash(state);
				c.hash(state);
			}
			Self::Number { value, signed, int } => {
				3.hash(state);
				value.to_bits().hash(state);
				signed.hash(state);
				int.hash(state);
			}
			Self::Dimension { value, signed, int, unit } => {
				4.hash(state);
				value.to_bits().hash(state);
				signed.hash(state);
				int.hash(state);
				unit.hash(state);
			}
			Self::Unrestricted(s) => {
				5.hash(state);
				s.hash(state);
			}
		};
	}
}
