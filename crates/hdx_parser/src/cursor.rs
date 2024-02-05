use std::ops::Range;

use hdx_atom::Atom;
use hdx_lexer::{LexerCheckpoint, Token};

use crate::{diagnostics, span::Span, Parser, Result};

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
	pub fn expect_ident_of(&mut self, expected_atom: &Atom) -> Result<()> {
		match self.cur() {
			Token::Ident(atom) => {
				if atom.eq_ignore_ascii_case(expected_atom) {
					self.advance();
					Ok(())
				} else {
					Err(diagnostics::ExpectedIdentOf(expected_atom.clone(), atom, self.span()))?
				}
			}
			token => Err(diagnostics::ExpectedIdent(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_function_of(&mut self, expected_atom: &Atom) -> Result<()> {
		match self.cur() {
			Token::Function(atom) => {
				if atom.eq_ignore_ascii_case(expected_atom) {
					self.advance();
					Ok(())
				} else {
					Err(diagnostics::ExpectedFunctionOf(expected_atom.clone(), atom, self.span()))?
				}
			}
			token => Err(diagnostics::ExpectedFunction(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_delim_of(&mut self, expected_ch: char) -> Result<()> {
		match self.cur() {
			Token::Delim(ch) => {
				if expected_ch != ch {
					Err(diagnostics::ExpectedDelimOf(expected_ch, ch, Span::new(self.pos(), self.pos())))?
				}
				self.advance();
				Ok(())
			}
			token => Err(diagnostics::ExpectedDelim(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_number_gte(&mut self, min: f32) -> Result<f32> {
		match self.cur() {
			Token::Number(n, _) => {
				if n < min {
					Err(diagnostics::NumberTooSmall(min, self.span()))?
				}
				self.advance();
				Ok(n)
			}
			token => Err(diagnostics::ExpectedNumber(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_number_in_range(&mut self, range: Range<f32>) -> Result<f32> {
		match self.cur() {
			Token::Number(n, _) => {
				if !range.contains(&n) {
					Err(diagnostics::NumberOutOfBounds(range.start, range.end, self.span()))?
				}
				self.advance();
				Ok(n)
			}
			token => Err(diagnostics::ExpectedNumber(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_int(&mut self) -> Result<i32> {
		match self.cur() {
			Token::Number(n, ty) => {
				if !ty.is_int() {
					Err(diagnostics::ExpectedInt(n, self.span()))?
				}
				self.advance();
				Ok(n as i32)
			}
			token => Err(diagnostics::ExpectedNumber(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_dimension_gte(&mut self, min: f32) -> Result<(f32, Atom)> {
		match self.cur() {
			Token::Dimension(n, atom, _) => {
				if n < min {
					Err(diagnostics::NumberTooSmall(min, self.span()))?
				}
				self.advance();
				Ok((n, atom))
			}
			token => Err(diagnostics::ExpectedDimension(token, self.span()))?,
		}
	}

	#[inline]
	pub fn expect_dimension_in_range(&mut self, range: Range<f32>) -> Result<(f32, Atom)> {
		match self.cur() {
			Token::Dimension(n, atom, _) => {
				if !range.contains(&n) {
					Err(diagnostics::NumberOutOfBounds(range.start, range.end, self.span()))?
				}
				self.advance();
				Ok((n, atom))
			}
			token => Err(diagnostics::ExpectedDimension(token, self.span()))?,
		}
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
