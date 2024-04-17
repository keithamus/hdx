mod constants;
mod private;
mod string_builder;
mod token;

use std::{collections::VecDeque, str::Chars};

use bitmask_enum::bitmask;
use bumpalo::Bump;
pub use token::{NumType, PairWise, Token, QuoteStyle};

#[derive(Debug, Clone)]
pub struct LexerCheckpoint<'a> {
	chars: Chars<'a>,
	token: Token,
}

#[bitmask(u8)]
pub(crate) enum Include {
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

	pub fn jump(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_back() {
			self.current.chars = checkpoint.chars;
			self.lookahead.clear();
			return checkpoint.token;
		}
		self.advance()
	}

	pub fn advance(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_front() {
			self.current.chars = checkpoint.chars;
			return checkpoint.token;
		}
		self.read_next_token()
	}

	pub fn advance_including_whitespace(&mut self) -> Token {
		self.include = Include::Whitespace;
		let token = self.advance();
		self.include = Include::none();
		token
	}

	pub fn advance_including_whitespace_and_comments(&mut self) -> Token {
		self.include = Include::all();
		let token = self.advance();
		self.include = Include::none();
		token
	}
}
