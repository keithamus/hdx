use hdx_lexer::{Include, Token};

use crate::{span::Span, Parser};

pub struct ParserCheckpoint {
	token: Token,
	warnings_pos: usize,
	errors_pos: usize,
}

impl<'a> Parser<'a> {
	#[inline]
	pub fn cur(&self) -> Token {
		self.token
	}

	#[inline]
	pub fn pos(&self) -> u32 {
		self.lexer.pos()
	}

	#[inline]
	pub fn span(&self) -> Span {
		Span::new(self.pos(), self.pos())
	}

	#[inline]
	pub fn peek(&mut self) -> Token {
		self.peek_with(self.lexer.include)
	}

	#[inline]
	pub fn peek_with(&mut self, inc: Include) -> Token {
		self.lexer.clone_with(inc).advance()
	}

	#[inline]
	pub fn peek_n(&mut self, n: u8) -> Token {
		self.peek_n_with(n, self.lexer.include)
	}

	#[inline]
	pub fn peek_n_with(&mut self, n: u8, inc: Include) -> Token {
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

	/// Should only be used in severe edge cases, for legacy parse modes
	pub fn legacy_peek_next_char(&self, n: usize) -> Option<char> {
		self.lexer.legacy_peek_next_char(n)
	}

	#[inline]
	pub fn next(&mut self) -> Token {
		self.token = self.lexer.advance();
		self.token
	}

	#[inline]
	pub fn next_with(&mut self, inc: Include) -> Token {
		self.token = self.lexer.clone_with(inc).advance();
		self.lexer.rewind(self.token);
		self.token
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint) {
		let ParserCheckpoint { token, warnings_pos, errors_pos } = checkpoint;
		self.lexer.rewind(token);
		self.token = token;
		self.warnings.truncate(warnings_pos);
		self.errors.truncate(errors_pos);
	}

	pub fn checkpoint(&self) -> ParserCheckpoint {
		ParserCheckpoint {
			token: self.token,
			warnings_pos: self.warnings.len(),
			errors_pos: self.errors.len(),
		}
	}
}
