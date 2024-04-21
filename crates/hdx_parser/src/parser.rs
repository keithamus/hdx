use bitmask_enum::bitmask;
use bumpalo::Bump;
use hdx_lexer::{Lexer, Token};
use miette::Error;

use crate::{diagnostics, span::Spanned, traits::Parse};

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
	fn enabled(&self, other: Features) -> bool {
		self.features.contains(other)
	}

	#[inline]
	pub fn is(&self, state: State) -> bool {
		self.state.contains(state)
	}

	pub fn parse_entirely_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		let (output, panicked) = match T::parse_spanned(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		if !matches!(self.next(), Token::Eof) {
			let span = self.span();
			loop {
				if matches!(self.next(), Token::Eof) {
					break;
				}
			}
			self.errors.push(diagnostics::ExpectedEnd(span.end(self.pos())).into());
		}
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	pub fn parse_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		let (output, panicked) = match T::parse_spanned(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	pub fn warn(&mut self, error: Error) {
		self.warnings.push(error);
	}
}
