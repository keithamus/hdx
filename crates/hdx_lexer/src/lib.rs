mod constants;
mod private;
mod string_builder;
mod token;

use std::{collections::VecDeque, str::Chars};

use bitmask_enum::bitmask;
use bumpalo::Bump;
pub use token::{NumType, PairWise, QuoteStyle, Token};

#[derive(Debug, Clone)]
pub struct LexerCheckpoint<'a> {
	chars: Chars<'a>,
	token: Token,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Include {
	Whitespace = 0b0001,
	Comments = 0b0010,
}

pub struct Lexer<'a> {
	allocator: &'a Bump,
	source: &'a str,
	current: LexerCheckpoint<'a>,
	lookahead: VecDeque<LexerCheckpoint<'a>>,
	include: Include,
}

impl<'a> Lexer<'a> {
	pub fn new(allocator: &'a Bump, source: &'a str) -> Self {
		let token = Token::default();
		let current = LexerCheckpoint { chars: source.chars(), token };
		Self { allocator, source, current, lookahead: VecDeque::with_capacity(4), include: Include::none() }
	}

	/// Remaining string from `Chars`
	fn remaining(&self) -> &'a str {
		self.current.chars.as_str()
	}

	/// Current position in file
	#[inline]
	pub fn pos(&self) -> u32 {
		(self.source.len() - self.remaining().len()) as u32
	}

	/// Creates a checkpoint storing the current lexer state.
	/// Use `rewind` to restore the lexer to the state stored in the checkpoint.
	pub fn checkpoint(&self) -> LexerCheckpoint<'a> {
		LexerCheckpoint { chars: self.current.chars.clone(), token: self.current.token.clone() }
	}

	/// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
	pub fn rewind(&mut self, checkpoint: LexerCheckpoint<'a>) {
		self.current = checkpoint;
		self.lookahead.clear();
	}

	/// Find the nth lookahead token lazily
	pub fn lookahead(&mut self, n: u8) -> &Token {
		let n = n as usize;
		debug_assert!(n > 0);

		if self.lookahead.len() > n - 1 {
			return &self.lookahead[n - 1].token;
		}

		let checkpoint = self.checkpoint();

		if let Some(checkpoint) = self.lookahead.back() {
			self.current = checkpoint.clone();
		}

		// reset the current token for `read_next_token`,
		// otherwise it will contain the token from
		// `self.current = checkpoint`
		self.current.token = Token::default();

		for _i in self.lookahead.len()..n {
			let peeked = self.read_next_token();
			self.lookahead.push_back(LexerCheckpoint { chars: self.current.chars.clone(), token: peeked });
		}

		self.current = checkpoint;

		&self.lookahead[n - 1].token
	}

	pub fn lookahead_with(&mut self, n: u8, inc: Include) -> &Token {
		self.include = inc;
		// we need to clear the lookahead if different characteristics
		// are used, as it will influence token count.
		self.lookahead.clear();
		self.lookahead(n);
		self.include = Include::none();
		self.lookahead(n)
	}

	pub fn jump(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_back() {
			self.current.chars = checkpoint.chars;
			self.lookahead.clear();
			return checkpoint.token;
		}
		self.advance()
	}

	#[inline]
	pub fn advance(&mut self) -> Token {
		self.lookahead.clear();
		self.read_next_token()
	}

	#[inline]
	pub fn advance_with(&mut self, inc: Include) -> Token {
		self.include = inc;
		let token = self.advance();
		self.include = Include::none();
		token
	}
}
