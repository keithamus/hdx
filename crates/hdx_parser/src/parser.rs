use std::mem::take;

use bitmask_enum::bitmask;
use bumpalo::Bump;
use hdx_lexer::{Cursor, Kind, KindSet, Lexer, Span};
use miette::Error;

use crate::{
	diagnostics,
	traits::{Parse, Peek},
	ParserReturn, Result, ToCursors,
};

#[derive(Debug)]
pub struct Parser<'a> {
	pub(crate) source_text: &'a str,

	pub(crate) lexer: Lexer<'a>,

	pub(crate) features: Features,

	pub(crate) errors: Vec<Error>,

	pub(crate) trivia: Vec<Cursor>,

	pub(crate) state: State,

	pub(crate) allocator: &'a Bump,

	pub skip: KindSet,

	pub stop: KindSet,

	#[cfg(debug_assertions)]
	pub(crate) last_cursor: Option<Cursor>,
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
#[derive(Default)]
pub enum State {
	Nested = 0b0000_0001,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(allocator: &'a Bump, source_text: &'a str, features: Features) -> Self {
		Self {
			source_text,
			lexer: Lexer::new(source_text),
			features,
			errors: vec![],
			trivia: vec![],
			state: State::none(),
			skip: KindSet::TRIVIA,
			stop: KindSet::NONE,
			allocator,
			#[cfg(debug_assertions)]
			last_cursor: None,
		}
	}

	#[inline]
	pub fn bump(&self) -> &'a Bump {
		self.allocator
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

	#[inline]
	pub fn set_skip(&mut self, skip: KindSet) -> KindSet {
		let old = self.skip;
		self.skip = skip;
		old
	}

	#[inline]
	pub fn set_stop(&mut self, stop: KindSet) -> KindSet {
		let old = self.stop;
		self.stop = stop;
		old
	}

	pub fn parse_entirely<T: Parse<'a> + ToCursors>(&mut self) -> ParserReturn<'a, T> {
		let output = match T::parse(self) {
			Ok(output) => Some(output),
			Err(error) => {
				self.errors.push(error);
				None
			}
		};
		if !self.at_end() && self.peek_next() != Kind::Eof {
			let start = self.offset();
			dbg!("Parse entirely saw the following remaining tokens...");
			loop {
				let cursor = self.next();
				self.trivia.push(cursor);
				dbg!(cursor);
				if cursor == Kind::Eof {
					break;
				}
			}
			self.errors.push(diagnostics::ExpectedEnd(Span::new(start, self.offset())).into());
		}
		ParserReturn::new(output, self.source_text, take(&mut self.errors), take(&mut self.trivia))
	}

	pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::parse(self)
	}

	pub fn peek<T: Peek<'a>>(&self) -> bool {
		T::peek(self)
	}

	pub fn parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self) {
			T::parse(self).map(Some)
		} else {
			Ok(None)
		}
	}

	pub fn try_parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::try_parse(self)
	}

	#[inline]
	pub fn parse_atom(&self, c: Cursor) -> hdx_atom::Atom {
		c.parse_atom(self.lexer.source(), self.allocator)
	}

	#[inline]
	pub fn parse_atom_lower(&self, c: Cursor) -> hdx_atom::Atom {
		c.parse_atom_lower(self.lexer.source(), self.allocator)
	}

	#[inline]
	pub fn parse_raw_str(&self, c: Cursor) -> &'a str {
		c.str_slice(self.lexer.source())
	}

	#[inline]
	pub fn parse_str(&self, c: Cursor) -> &str {
		c.parse_str(self.lexer.source(), self.allocator)
	}

	#[inline]
	pub fn eq_ignore_ascii_case(&self, c: Cursor, other: &'static str) -> bool {
		c.eq_ignore_ascii_case(self.lexer.source(), other)
	}
}
