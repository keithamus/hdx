use hdx_lexer::{LexerCheckpoint, Token};

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
	pub fn cur(&self) -> Token {
		self.token.clone()
	}

	#[inline]
	pub fn pos(&self) -> u32 {
		self.lexer.pos()
	}

	#[inline]
	pub fn span(&self) -> Span {
		Span::new(self.prev_pos, self.lexer.pos())
	}

	#[inline]
	pub fn peek(&mut self) -> &Token {
		self.lexer.lookahead(1)
	}

	#[inline]
	pub fn advance_including_whitespace_and_comments(&mut self) {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance_including_whitespace_and_comments();
	}

	#[inline]
	pub fn advance_including_whitespace(&mut self) {
		self.prev_pos = self.lexer.pos();
		self.token = self.lexer.advance_including_whitespace();
	}

	#[inline]
	pub fn advance(&mut self) -> Span {
		let span = self.span();
		self.prev_pos = span.end;
		self.token = self.lexer.advance();
		span
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
