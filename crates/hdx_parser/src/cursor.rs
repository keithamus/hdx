use hdx_lexer::{Include, LexerCheckpoint, Token};

use crate::{span::Span, Parser};

pub struct ParserCheckpoint<'a> {
	lexer: LexerCheckpoint<'a>,
	token: Token,
	prev_pos: u32,
	warnings_pos: usize,
	errors_pos: usize,
}

impl<'a> Parser<'a> {
	#[inline]
	pub fn cur(&self) -> &Token {
		&self.token
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
	pub fn peek(&mut self) -> &Token {
		self.lexer.lookahead(1)
	}

	#[inline]
	pub fn peek_with(&mut self, inc: Include) -> &Token {
		self.lexer.lookahead_with(1, inc)
	}

	#[inline]
	pub fn peek_n(&mut self, n: u8) -> &Token {
		self.lexer.lookahead(n)
	}

	/// Should only be used in severe edge cases, for legacy parse modes
	pub fn legacy_peek_next_char(&self, n: usize) -> Option<char> {
		self.lexer.legacy_peek_next_char(n)
	}

	#[inline]
	pub fn advance(&mut self) {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance()
	}

	#[inline]
	pub fn advance_with(&mut self, inc: Include) {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance_with(inc);
	}

	#[inline]
	pub fn next(&mut self) -> &Token {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance();
		&self.token
	}

	#[inline]
	pub fn next_with(&mut self, inc: Include) -> &Token {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance_with(inc);
		&self.token
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint<'a>) {
		let ParserCheckpoint { lexer, prev_pos, token, warnings_pos, errors_pos } = checkpoint;
		self.lexer.rewind(lexer);
		self.token = token;
		self.prev_pos = prev_pos;
		self.warnings.truncate(warnings_pos);
		self.errors.truncate(errors_pos);
	}

	pub fn checkpoint(&self) -> ParserCheckpoint<'a> {
		ParserCheckpoint {
			lexer: self.lexer.checkpoint(),
			prev_pos: self.prev_pos,
			token: self.token.clone(),
			warnings_pos: self.warnings.len(),
			errors_pos: self.errors.len(),
		}
	}
}
