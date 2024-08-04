use hdx_lexer::{Include, Kind, LexerCheckpoint, Token};

use crate::Parser;

pub struct ParserCheckpoint {
	checkpoint: LexerCheckpoint,
	errors_pos: u8,
}

impl<'a> Parser<'a> {
	#[inline]
	pub fn offset(&self) -> u32 {
		self.lexer.offset()
	}

	#[inline]
	pub fn at_end(&self) -> bool {
		self.lexer.at_end() || self.peek_next().kind() == Kind::Eof
	}

	#[inline]
	pub fn hop(&mut self, token: Token) {
		self.lexer.hop(token);
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint) {
		let ParserCheckpoint { checkpoint, errors_pos } = checkpoint;
		self.lexer.rewind(checkpoint);
		self.errors.truncate(errors_pos as usize);
	}

	pub fn checkpoint(&self) -> ParserCheckpoint {
		ParserCheckpoint { checkpoint: self.lexer.checkpoint(), errors_pos: self.errors.len() as u8 }
	}

	#[inline]
	pub(crate) fn peek_next(&self) -> Token {
		self.lexer.clone().advance()
	}

	#[inline]
	pub(crate) fn peek_n(&self, n: u8) -> Token {
		self.peek_n_with(n, self.lexer.include)
	}

	#[inline]
	pub(crate) fn peek_n_with(&self, n: u8, inc: Include) -> Token {
		let mut lex = self.lexer.clone_with(inc);
		let mut remaining = n;
		loop {
			let token = lex.advance();
			remaining -= 1;
			if remaining == 0 {
				return token;
			}
		}
	}

	#[inline]
	pub(crate) fn next(&mut self) -> Token {
		let token = self.lexer.advance();

		#[cfg(debug_assertions)]
		if let Some(last_token) = self.last_token {
			debug_assert!(last_token != token, "Detected a next loop, {:?} was fetched twice", token);
		}
		#[cfg(debug_assertions)]
		if token.kind() == hdx_lexer::Kind::Eof {
			self.last_token = None;
		} else {
			self.last_token = Some(token);
		}

		token
	}

	#[inline]
	pub(crate) fn next_with(&mut self, inc: Include) -> Token {
		let token = self.lexer.clone_with(inc).advance();

		#[cfg(debug_assertions)]
		if let Some(last_token) = self.last_token {
			debug_assert!(last_token != token, "Detected a next loop {:?} was fetched twice", token);
		}
		#[cfg(debug_assertions)]
		if token.kind() == hdx_lexer::Kind::Eof {
			self.last_token = None;
		} else {
			self.last_token = Some(token);
		}

		self.lexer.hop(token);
		token
	}
}
