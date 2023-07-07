use std::ops::Range;

use hdx_lexer::{Kind, LexerCheckpoint, Span, Token};

use crate::{diagnostics, Atom, Error, Parser, Result};

pub struct ParserCheckpoint<'a> {
	lexer: LexerCheckpoint<'a>,
	token: Token,
	prev_span: Span,
	warnings_pos: usize,
	errors_pos: usize,
}

impl<'a> Parser<'a> {
	#[inline]
	pub(crate) fn cur(&self) -> &Token {
		&self.token
	}

	#[inline]
	pub(crate) fn cur_atom(&self) -> Option<Atom> {
		self.token.value.as_atom()
	}

	#[inline]
	pub(crate) fn cur_atom_lower(&self) -> Option<Atom> {
		self.token.value.as_atom_lower()
	}

	#[inline]
	pub(crate) fn cur_char(&self) -> Option<char> {
		self.token.value.as_char()
	}

	pub(crate) fn at(&self, kind: Kind) -> bool {
		self.cur().kind == kind
	}

	pub(crate) fn expect(&mut self, kind: Kind) -> Result<()> {
		self.expect_without_advance(kind)?;
		self.advance();
		Ok(())
	}

	#[inline]
	pub(crate) fn expect_ident_cased(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::Ident)?.value.as_atom().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_ident(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::Ident)?.value.as_atom_lower().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_ident_of(&mut self, atom: Atom) -> Result<()> {
		let ident = self.expect_without_advance(Kind::Ident)?.value.as_atom_lower().unwrap();
		if atom != ident {
			Err(diagnostics::ExpectedIdent(atom, ident, self.cur().span))?
		}
		self.advance();
		Ok(())
	}

	#[inline]
	pub(crate) fn expect_function(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::Function)?.value.as_atom_lower().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_function_of(&mut self, atom: Atom) -> Result<()> {
		let ident = self.expect_without_advance(Kind::Function)?.value.as_atom_lower().unwrap();
		if atom != ident {
			Err(diagnostics::ExpectedFunction(atom, ident, self.cur().span))?
		}
		self.advance();
		Ok(())
	}

	#[inline]
	pub(crate) fn expect_at_keyword(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::AtKeyword)?.value.as_atom_lower().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_at_keyword_of(&mut self, atom: Atom) -> Result<()> {
		let ident = self.expect_without_advance(Kind::AtKeyword)?.value.as_atom_lower().unwrap();
		if atom != ident {
			Err(diagnostics::ExpectedAtKeyword(atom, ident, self.cur().span))?
		}
		self.advance();
		Ok(())
	}

	#[inline]
	pub(crate) fn expect_hash(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::Hash)?.value.as_atom().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_string(&mut self) -> Result<Atom> {
		let atom = self.expect_without_advance(Kind::String)?.value.as_atom().unwrap();
		self.advance();
		Ok(atom)
	}

	#[inline]
	pub(crate) fn expect_delim(&mut self) -> Result<char> {
		let char = self.expect_without_advance(Kind::Delim)?.value.as_char().unwrap();
		self.advance();
		Ok(char)
	}

	#[inline]
	pub(crate) fn expect_delim_of(&mut self, ch: char) -> Result<()> {
		if ch != self.expect_without_advance(Kind::Delim)?.value.as_char().unwrap() {
			Err(diagnostics::UnexpectedDelim(ch, self.cur().span))?
		}
		self.advance();
		Ok(())
	}

	#[inline]
	pub(crate) fn expect_number(&mut self) -> Result<f32> {
		let n = self.expect_without_advance(Kind::Number)?.value.as_f32().unwrap();
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_number_gte(&mut self, min: f32) -> Result<f32> {
		let n = self.expect_without_advance(Kind::Number)?.value.as_f32().unwrap();
		if n < min {
			Err(diagnostics::NumberTooSmall(min, self.cur().span))?
		}
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_number_in_range(&mut self, range: Range<f32>) -> Result<f32> {
		let n = self.expect_without_advance(Kind::Number)?.value.as_f32().unwrap();
		if !range.contains(&n) {
			Err(diagnostics::NumberOutOfBounds(range.start, range.end, self.cur().span))?
		}
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_int(&mut self) -> Result<i32> {
		self.expect_without_advance(Kind::Number)?;
		if !self.cur().value.is_int() {
			Err(diagnostics::DisallowedFloat(self.cur().value.as_i32().unwrap(), self.cur().span))?
		}
		let n = self.cur().value.as_i32().unwrap();
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_percentage(&mut self) -> Result<f32> {
		let n = self.expect_without_advance(Kind::Percentage)?.value.as_f32().unwrap();
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_percentage_gte(&mut self, min: f32) -> Result<f32> {
		let n = self.expect_without_advance(Kind::Percentage)?.value.as_f32().unwrap();
		if n < min {
			Err(diagnostics::NumberTooSmall(min, self.cur().span))?
		}
		self.advance();
		Ok(n)
	}

	#[inline]
	pub(crate) fn expect_dimension(&mut self) -> Result<(f32, Atom)> {
		let value = &self.expect_without_advance(Kind::Dimension)?.value;
		let (n, atom) = (value.as_f32().unwrap(), value.as_atom_lower().unwrap());
		self.advance();
		Ok((n, atom))
	}

	#[inline]
	pub(crate) fn expect_dimension_gte(&mut self, min: f32) -> Result<(f32, Atom)> {
		let value = &self.expect_without_advance(Kind::Dimension)?.value;
		let (n, atom) = (value.as_f32().unwrap(), value.as_atom_lower().unwrap());
		if n < min {
			Err(diagnostics::NumberTooSmall(min, self.cur().span))?
		}
		self.advance();
		Ok((n, atom))
	}

	#[inline]
	pub(crate) fn expect_dimension_in_range(&mut self, range: Range<f32>) -> Result<(f32, Atom)> {
		let value = &self.expect_without_advance(Kind::Dimension)?.value;
		let (n, atom) = (value.as_f32().unwrap(), value.as_atom_lower().unwrap());
		if !range.contains(&n) {
			Err(diagnostics::NumberOutOfBounds(range.start, range.end, self.cur().span))?
		}
		self.advance();
		Ok((n, atom))
	}

	#[inline]
	pub(crate) fn expect_without_advance(&mut self, kind: Kind) -> Result<&Token> {
		if !self.at(kind) {
			let range = self.cur().span;
			Err::<(), Error>(diagnostics::ExpectedToken(kind, self.cur().kind, range).into())?;
		}
		Ok(self.cur())
	}

	#[inline]
	pub(crate) fn peek_including_trivia(&mut self) -> &Token {
		self.lexer.lookahead(1)
	}

	pub(crate) fn peek(&mut self) -> &Token {
		let mut i: u8 = 0;
		loop {
			i += 1;
			if !self.lexer.lookahead(i).is_trivia() {
				return self.lexer.lookahead(i);
			}
		}
	}

	#[inline]
	pub(crate) fn next_token_include_comments(&mut self) {
		self.prev_span = self.token.span;
		self.token = self.lexer.next_token();
	}

	pub(crate) fn next_token(&mut self) {
		self.prev_span = self.token.span;
		loop {
			let token = self.lexer.next_token();
			if token.kind != Kind::Comment {
				self.token = token;
				return;
			}
		}
	}

	pub(crate) fn advance(&mut self) {
		self.prev_span = self.token.span;
		loop {
			let token = self.lexer.next_token();
			dbg!("next token is ", &token);
			if !token.is_trivia() {
				self.token = token;
				return;
			}
		}
	}

	#[inline]
	pub(crate) fn skip_trivia(&mut self) {
		if self.cur().is_trivia() {
			self.advance();
		}
	}

	pub(crate) fn rewind(&mut self, checkpoint: ParserCheckpoint<'a>) {
		let ParserCheckpoint { lexer, token, prev_span, warnings_pos, errors_pos } = checkpoint;
		self.lexer.rewind(lexer);
		self.token = token;
		self.prev_span = prev_span;
		self.warnings.truncate(warnings_pos);
		self.errors.truncate(errors_pos);
	}

	pub(crate) fn checkpoint(&self) -> ParserCheckpoint<'a> {
		ParserCheckpoint {
			lexer: self.lexer.checkpoint(),
			token: self.token.clone(),
			prev_span: self.prev_span,
			warnings_pos: self.warnings.len(),
			errors_pos: self.errors.len(),
		}
	}
}
