use hdx_lexer::{Include, Kind, Span, Token};

use crate::{unexpected, Parser, Result};

pub struct ParserCheckpoint {
	token: Token,
	warnings_pos: u8,
	errors_pos: u8,
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
	pub fn peek_next(&mut self) -> Token {
		self.peek_with(self.lexer.include)
	}

	#[inline]
	pub fn peek_kind(&mut self, kind: Kind) -> Option<Token> {
		let token = self.peek_with(self.lexer.include);
		if token.kind() == kind {
			Some(token)
		} else {
			None
		}
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
	pub fn next_kind(&mut self, kind: Kind) -> Result<Token> {
		self.token = self.lexer.advance();
		if self.token.kind() == kind {
			Ok(self.token)
		} else {
			unexpected!(self, self.token)
		}
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
		self.warnings.truncate(warnings_pos as usize);
		self.errors.truncate(errors_pos as usize);
	}

	pub fn checkpoint(&self) -> ParserCheckpoint {
		ParserCheckpoint {
			token: self.token,
			warnings_pos: self.warnings.len() as u8,
			errors_pos: self.errors.len() as u8,
		}
	}
}
