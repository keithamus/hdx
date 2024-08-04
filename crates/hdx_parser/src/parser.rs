use bitmask_enum::bitmask;
use bumpalo::Bump;
use hdx_lexer::{Include, Kind, Lexer, Spanned, Token};
use miette::Error;
use std::mem::take;

use crate::{
	diagnostics,
	traits::{Parse, Peek},
	Result,
};

pub struct Parser<'a> {
	pub(crate) lexer: Lexer<'a>,

	pub(crate) features: Features,

	pub(crate) warnings: std::vec::Vec<Error>,

	pub(crate) errors: std::vec::Vec<Error>,

	pub(crate) token: Token,

	pub(crate) state: State,

	pub(crate) prev_pos: u32,

	pub(crate) allocator: &'a Bump,
}

#[bitmask(u8)]
pub enum Features {}

impl Default for Features {
	fn default() -> Self {
		Self::none()
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum State {
	Nested = 0b0000_0001,

	// Stop Tokens for some algorithms
	StopOnSemicolon = 0b1000_0000,
	StopOnComma = 0b1100_0000,
}

pub struct ParserReturn<T> {
	pub output: Option<T>,
	pub errors: std::vec::Vec<Error>,
	pub warnings: std::vec::Vec<Error>,
	pub panicked: bool,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(allocator: &'a Bump, source_text: &'a str, features: Features) -> Self {
		Self {
			lexer: Lexer::new(source_text, Include::none()),
			features,
			warnings: std::vec::Vec::new(),
			errors: std::vec::Vec::new(),
			token: Token::default(),
			state: State::none(),
			prev_pos: 0,
			allocator,
		}
	}

	#[inline]
	pub fn new_vec<T>(&self) -> crate::Vec<'a, T> {
		crate::Vec::new_in(self.allocator)
	}

	#[inline]
	fn enabled(&self, other: Features) -> bool {
		self.features.contains(other)
	}

	#[inline]
	pub fn is(&self, state: State) -> bool {
		self.state.contains(state)
	}

	pub fn parse_entirely_with<T: Parse<'a>>(&mut self) -> ParserReturn<Spanned<T>> {
		let (output, panicked) = match T::parse_spanned(self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		if !self.at_end() {
			let span = self.cur().span();
			loop {
				if matches!(self.next().kind(), Kind::Eof) {
					break;
				}
			}
			self.errors.push(diagnostics::ExpectedEnd(span.end(self.pos())).into());
		}
		ParserReturn { output, warnings: take(&mut self.warnings), errors: take(&mut self.errors), panicked }
	}

	pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::parse(self)
	}

	pub fn parse_with<T: Parse<'a>>(&mut self, inc: Include) -> Result<T> {
		let old_inc = self.lexer.include;
		self.lexer = self.lexer.clone_with(inc);
		let res = T::parse(self);
		self.lexer = self.lexer.clone_with(old_inc);
		res
	}

	pub fn parse_spanned<T: Parse<'a>>(&mut self) -> Result<Spanned<T>> {
		T::parse_spanned(self)
	}

	pub fn peek<T: Peek<'a>>(&self) -> Option<Token> {
		T::peek(self)
	}

	pub fn peek_with<T: Peek<'a>>(&mut self, inc: Include) -> Option<Token> {
		let old_inc = self.lexer.include;
		self.lexer = self.lexer.clone_with(inc);
		let token = T::peek(self);
		self.lexer = self.lexer.clone_with(old_inc);
		token
	}

	pub fn try_parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::try_parse(self)
	}

	pub fn try_parse_spanned<T: Parse<'a>>(&mut self) -> Option<Spanned<T>> {
		let checkpoint = self.checkpoint();
		if let Ok(res) = T::parse_spanned(self) {
			Some(res)
		} else {
			self.rewind(checkpoint);
			None
		}
	}

	#[inline]
	pub fn warn(&mut self, error: Error) {
		self.warnings.push(error);
	}

	#[inline]
	pub fn parse_atom(&self, tok: Token) -> hdx_atom::Atom {
		self.lexer.parse_atom(tok, self.allocator)
	}

	#[inline]
	pub fn parse_atom_lower(&self, tok: Token) -> hdx_atom::Atom {
		self.lexer.parse_atom_lower(tok, self.allocator)
	}

	#[inline]
	pub fn parse_number(&self, tok: Token) -> f32 {
		self.lexer.parse_number(tok)
	}

	#[inline]
	pub fn parse_raw_str(&self, tok: Token) -> &'a str {
		self.lexer.parse_raw_str(tok)
	}

	#[inline]
	pub fn parse_str(&self, tok: Token) -> &'a str {
		self.lexer.parse_str(tok, self.allocator)
	}
}
