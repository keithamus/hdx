mod constants;
mod kind;
mod private;
mod span;
mod string_builder;
mod token;

use std::{collections::VecDeque, str::Chars};

use oxc_allocator::Allocator;
pub use span::Span;
pub use token::{PairWise, Token, TokenValue};

pub use self::kind::Kind;

#[derive(Debug, Clone)]
pub struct LexerCheckpoint<'a> {
	chars: Chars<'a>,
	token: Token,
}

pub struct Lexer<'a> {
	allocator: &'a Allocator,
	source: &'a str,
	current: LexerCheckpoint<'a>,
	lookahead: VecDeque<LexerCheckpoint<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(allocator: &'a Allocator, source: &'a str) -> Self {
		let token = Token::default();
		let current = LexerCheckpoint { chars: source.chars(), token };
		Self { allocator, source, current, lookahead: VecDeque::with_capacity(4) }
	}

	/// Remaining string from `Chars`
	fn remaining(&self) -> &'a str {
		self.current.chars.as_str()
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
			let kind = self.read_next_token();
			let peeked = self.finish_next(kind);
			self.lookahead
				.push_back(LexerCheckpoint { chars: self.current.chars.clone(), token: peeked });
		}

		self.current = checkpoint;

		&self.lookahead[n - 1].token
	}

	pub fn jump_token(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_back() {
			self.current.chars = checkpoint.chars;
			self.lookahead.clear();
			return checkpoint.token;
		}
		self.next_token()
	}

	pub fn next_token(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_front() {
			self.current.chars = checkpoint.chars;
			return checkpoint.token;
		}
		let kind = self.read_next_token();
		self.finish_next(kind)
	}
}
