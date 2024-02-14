use bitmask_enum::bitmask;
use hdx_lexer::{Lexer, Token};
use miette::{Error, Result};
use oxc_allocator::Allocator;

use crate::{diagnostics, span::Spanned, traits::Parse};

pub struct Parser<'a> {
	pub(crate) lexer: Lexer<'a>,

	pub(crate) features: Features,

	pub(crate) warnings: std::vec::Vec<Error>,

	pub(crate) errors: std::vec::Vec<Error>,

	pub(crate) token: Token,

	pub(crate) state: State,

	pub(crate) prev_pos: u32,

	pub(crate) allocator: &'a Allocator,
}

#[bitmask(u8)]
pub enum Features {
	Sloppy = 0x01,
}

impl Default for Features {
    fn default() -> Self {
        Self::none()
    }
}

#[bitmask(u8)]
pub enum State {
	Nested = 0b0000_0001,

	// Stop Tokens for some algorithms
	StopOnSemicolon =   0b1000_0000,
	StopOnComma =       0b1100_0000,
}

pub struct ParserReturn<T> {
	pub output: Option<T>,
	pub errors: std::vec::Vec<Error>,
	pub warnings: std::vec::Vec<Error>,
	pub panicked: bool,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(allocator: &'a Allocator, source_text: &'a str, features: Features) -> Self {
		Self {
			lexer: Lexer::new(allocator, source_text),
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
	pub fn boxup<T>(&self, value: T) -> oxc_allocator::Box<'a, T> {
		oxc_allocator::Box(self.allocator.alloc(value))
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
	pub fn set(&mut self, state: State) {
		self.state |= state;
	}

	#[inline]
	pub fn unset(&mut self, state: State) {
		self.state &= !state;
	}

	#[inline]
	pub fn with_state<F, T>(&mut self, state: State, call: F) -> T where F: FnOnce(&mut Parser) -> T {
		self.set(state);
		let ret = call(self);
		self.unset(state);
		ret
	}

	pub fn parse_entirely_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		self.advance();
		let (output, panicked) = match T::parse(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		if !matches!(self.cur(), Token::Eof) {
			let span = self.span();
			loop {
				self.advance();
				if matches!(self.cur(), Token::Eof) {
					break;
				}
			}
			self.errors.push(diagnostics::ExpectedEnd(span.end(self.pos())).into());
		}
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	pub fn parse_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		self.advance();
		let (output, panicked) = match T::parse(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	pub fn parse_comma_list_of<T: Parse<'a>>(&mut self) -> Result<oxc_allocator::Vec<'a, Spanned<T>>> {
		let mut vec = self.new_vec();
		let mut last_kind;
		loop {
			vec.push(T::parse(self)?);
			match self.cur() {
				Token::Comma => {
					self.advance();
					last_kind = Token::Comma;
				}
				t => {
					last_kind = t;
					break;
				}
			}
		}
		if matches!(last_kind, Token::Comma) {
			let warn: Error = diagnostics::WarnTrailing(self.cur(), self.span()).into();
			if !self.enabled(Features::Sloppy) {
				Err(warn)?;
			}
		}
		Ok(vec)
	}

	pub fn warn(&mut self, error: Error) {
		self.warnings.push(error);
	}
}
