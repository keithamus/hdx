use bitmask_enum::bitmask;
use bumpalo::Bump;
use hdx_lexer::{Include, Kind, Lexer, Span, Spanned, Token};
use miette::Error;
use smallvec::{smallvec, SmallVec};
use std::mem::take;

use crate::{
	diagnostics,
	traits::{Parse, Peek},
	Result,
};

pub struct Parser<'a> {
	pub(crate) lexer: Lexer<'a>,

	pub(crate) features: Features,

	pub(crate) errors: SmallVec<[Error; 0]>,

	pub(crate) state: State,

	pub(crate) allocator: &'a Bump,

	#[cfg(debug_assertions)]
	pub(crate) last_token: Option<Token>,
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
	pub errors: SmallVec<[Error; 0]>,
	pub panicked: bool,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(allocator: &'a Bump, source_text: &'a str, features: Features) -> Self {
		Self {
			lexer: Lexer::new(source_text, Include::none()),
			features,
			errors: smallvec![],
			state: State::none(),
			allocator,
			#[cfg(debug_assertions)]
			last_token: None,
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

	#[inline]
	pub fn set_state(&mut self, state: State) -> State {
		let old = self.state;
		self.state = state;
		old
	}

	pub fn parse_entirely<T: Parse<'a>>(&mut self) -> ParserReturn<Spanned<T>> {
		let (output, panicked) = match T::parse_spanned(self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		if !self.at_end() && self.peek_next().kind() != Kind::Eof {
			let start = self.offset();
			dbg!("Parse entirely saw the following remaining tokens...", self.peek_next());
			loop {
				let token = self.next();
				dbg!(token);
				if matches!(token.kind(), Kind::Eof) {
					break;
				}
			}
			self.errors.push(diagnostics::ExpectedEnd(Span::new(start, self.offset())).into());
		}
		ParserReturn { output, errors: take(&mut self.errors), panicked }
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

	pub fn parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self).is_some() {
			T::parse(self).map(Some)
		} else {
			Ok(None)
		}
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
	pub fn parse_atom(&self, token: Token) -> hdx_atom::Atom {
		self.lexer.parse_atom(token, self.allocator)
	}

	#[inline]
	pub fn parse_atom_lower(&self, token: Token) -> hdx_atom::Atom {
		self.lexer.parse_atom_lower(token, self.allocator)
	}

	#[inline]
	pub fn parse_number(&self, token: Token) -> f32 {
		self.lexer.parse_number(token)
	}

	#[inline]
	pub fn parse_raw_str(&self, token: Token) -> &'a str {
		self.lexer.parse_raw_str(token)
	}

	#[inline]
	pub fn parse_str(&self, token: Token) -> &'a str {
		self.lexer.parse_str(token, self.allocator)
	}
}
