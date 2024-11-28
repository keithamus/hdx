use hdx_lexer::{Cursor, Kind, SourceOffset, Span, Token};

use crate::Parser;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserCheckpoint {
	checkpoint: Cursor,
	errors_pos: u8,
	trivia_pos: u16,
}

impl ParserCheckpoint {
	pub fn token(&self) -> Token {
		self.checkpoint.token()
	}

	pub fn span(&self) -> Span {
		self.checkpoint.span()
	}
}

impl From<ParserCheckpoint> for Token {
	fn from(value: ParserCheckpoint) -> Self {
		value.token()
	}
}

impl From<ParserCheckpoint> for Kind {
	fn from(value: ParserCheckpoint) -> Self {
		value.token().kind()
	}
}

impl From<ParserCheckpoint> for Span {
	fn from(value: ParserCheckpoint) -> Self {
		value.span()
	}
}

impl<'a> Parser<'a> {
	#[inline(always)]
	pub fn offset(&self) -> SourceOffset {
		self.lexer.offset()
	}

	#[inline(always)]
	pub fn at_end(&self) -> bool {
		self.lexer.at_end()
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint) {
		let ParserCheckpoint { checkpoint, errors_pos, trivia_pos } = checkpoint;
		self.lexer.rewind(checkpoint);
		self.errors.truncate(errors_pos as usize);
		self.trivia.truncate(trivia_pos as usize);
		self.last_cursor = None;
	}

	#[inline]
	pub fn checkpoint(&self) -> ParserCheckpoint {
		ParserCheckpoint {
			checkpoint: self.lexer.checkpoint(),
			errors_pos: self.errors.len() as u8,
			trivia_pos: self.trivia.len() as u16,
		}
	}

	#[inline]
	pub fn next_is_stop(&self) -> bool {
		let mut lexer = self.lexer.clone();
		loop {
			let t = lexer.advance();
			if t.kind() != self.skip {
				return t.kind() == self.stop;
			}
		}
	}

	#[inline]
	pub(crate) fn peek_next(&self) -> Cursor {
		let mut lexer = self.lexer.clone();
		loop {
			let offset = lexer.offset();
			let t = lexer.advance();
			if t == Kind::Eof || t != self.skip {
				return t.with_cursor(offset);
			}
		}
	}

	#[inline]
	pub(crate) fn peek_next_including_whitespace(&self) -> Cursor {
		let mut lexer = self.lexer.clone();
		loop {
			let offset = lexer.offset();
			let t = lexer.advance();
			if t == Kind::Eof || t == Kind::Whitespace || t != self.skip {
				return t.with_cursor(offset);
			}
		}
	}

	pub fn peek_n(&self, n: u8) -> Cursor {
		let mut lex = self.lexer.clone();
		let mut remaining = n;
		loop {
			let offset = lex.offset();
			let t = lex.advance();
			if t == Kind::Eof {
				return t.with_cursor(offset);
			}
			if t != self.skip {
				remaining -= 1;
				if remaining == 0 {
					return t.with_cursor(offset);
				}
			}
		}
	}

	pub fn consume_trivia(&mut self) {
		loop {
			let offset = self.lexer.offset();
			let c = self.lexer.advance().with_cursor(offset);
			if c == Kind::Eof {
				return;
			} else if c == self.skip {
				self.trivia.push(c)
			} else {
				self.lexer.rewind(c);
				return;
			}
		}
	}

	pub fn next(&mut self) -> Cursor {
		let mut c;
		let mut offset;
		loop {
			offset = self.offset();
			c = self.lexer.advance().with_cursor(offset);
			if c == Kind::Eof || c != self.skip {
				break;
			}
			self.trivia.push(c)
		}

		#[cfg(debug_assertions)]
		if let Some(last_cursor) = self.last_cursor {
			debug_assert!(last_cursor != c, "Detected a next loop, {:?} was fetched twice", c);
		}
		#[cfg(debug_assertions)]
		if c == hdx_lexer::Kind::Eof {
			self.last_cursor = None;
		} else {
			self.last_cursor = Some(c);
		}

		c
	}
}
