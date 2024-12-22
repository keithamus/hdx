use css_lexer::{Cursor, Kind, Span, Token};

/// Represents a point during the [Parser's][crate::Parser] lifecycle; retaining state that can then be rewound.
///
/// Don't use this directly, instead retrieve a checkpoint with [Parser::checkpoint()][crate::Parser::checkpoint] and
/// rewind the parser to a checkpoint with [Parser::rewind()][crate::Parser::rewind()].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserCheckpoint {
	pub(crate) cursor: Cursor,
	pub(crate) errors_pos: u8,
	pub(crate) trivia_pos: u16,
}

impl From<ParserCheckpoint> for Token {
	fn from(value: ParserCheckpoint) -> Self {
		value.cursor.token()
	}
}

impl From<ParserCheckpoint> for Kind {
	fn from(value: ParserCheckpoint) -> Self {
		value.cursor.token().kind()
	}
}

impl From<ParserCheckpoint> for Span {
	fn from(value: ParserCheckpoint) -> Self {
		value.cursor.span()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ParserCheckpoint>(), 16);
	}
}
