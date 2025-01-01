use crate::{Kind, Token};

/// Represents either the left or right [Kind] of a [PairWise] set.
///
/// Certain kinds have a [PairWise] equivalent:
///  - [Kind::LeftParen] has [Kind::RightParen]
///  - [Kind::LeftCurly] has [Kind::RightCurly]
///  - [Kind::LeftSquare] has [Kind::RightSquare]
///
/// The [PairWise] enum represents either one of the above listed [Kinds][Kind]. So for example [PairWise::Paren]
/// represents both the [Kind::LeftParen] and [Kind::RightParen].
///
/// # Example
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("(a)");
/// {
///		let token = lexer.advance();
///		assert_eq!(token, PairWise::Paren);
///		assert_eq!(token, Kind::LeftParen);
///		let pair: PairWise = token.to_pairwise().unwrap();
///		let mut close_token;
///		loop {
///			close_token = lexer.advance();
///			if close_token == pair {
///				break;
///			}
///		}
///		assert_eq!(close_token, Kind::RightParen);
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PairWise {
	Paren,
	Curly,
	Square,
}

impl PairWise {
	pub fn from_token(token: &Token) -> Option<Self> {
		match token.kind() {
			Kind::LeftParen | Kind::Function => Some(Self::Paren),
			Kind::LeftCurly => Some(Self::Curly),
			Kind::LeftSquare => Some(Self::Square),
			Kind::RightParen => Some(Self::Paren),
			Kind::RightCurly => Some(Self::Curly),
			Kind::RightSquare => Some(Self::Square),
			_ => None,
		}
	}

	pub fn start(&self) -> Token {
		match self {
			Self::Paren => Token::LEFT_PAREN,
			Self::Curly => Token::LEFT_CURLY,
			Self::Square => Token::LEFT_SQUARE,
		}
	}

	pub fn end(&self) -> Token {
		match self {
			Self::Paren => Token::RIGHT_PAREN,
			Self::Curly => Token::RIGHT_CURLY,
			Self::Square => Token::RIGHT_SQUARE,
		}
	}
}

impl PartialEq<Kind> for PairWise {
	fn eq(&self, other: &Kind) -> bool {
		self.end() == *other || self.start() == *other
	}
}

#[test]
fn test_partial_eq_kind() {
	assert_eq!(PairWise::Paren, Kind::LeftParen);
	assert_eq!(PairWise::Paren, Kind::RightParen);
	assert_eq!(PairWise::Square, Kind::LeftSquare);
	assert_eq!(PairWise::Square, Kind::RightSquare);
	assert_eq!(PairWise::Curly, Kind::LeftCurly);
	assert_eq!(PairWise::Curly, Kind::RightCurly);
}
