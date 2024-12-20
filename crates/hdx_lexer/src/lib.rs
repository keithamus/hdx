mod comment_style;
mod constants;
mod cursor;
mod dimension_unit;
mod kind;
mod kindset;
mod pairwise;
mod private;
mod quote_style;
mod source_offset;
mod span;
mod token;
mod whitespace_style;

use bitmask_enum::bitmask;

pub use comment_style::CommentStyle;
pub use cursor::Cursor;
pub use dimension_unit::DimensionUnit;
pub use kind::Kind;
pub use kindset::KindSet;
pub use pairwise::PairWise;
pub use quote_style::QuoteStyle;
pub use source_offset::SourceOffset;
pub use span::{Span, SpanContents, Spanned};
pub use token::Token;
pub use whitespace_style::WhitespaceStyle;

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum Feature {
	SingleLineComments = 0b0001,
	CombinedWhitespace = 0b0010,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Lexer<'a> {
	source: &'a str,
	offset: SourceOffset,
	token: Token,
	features: Feature,
}

impl<'a> Lexer<'a> {
	#[inline]
	pub fn new(source: &'a str) -> Self {
		Self { source, ..Default::default() }
	}

	#[inline]
	pub fn new_with_features(source: &'a str, features: Feature) -> Self {
		Self { source, features, ..Default::default() }
	}

	#[inline(always)]
	pub fn source(&self) -> &'a str {
		self.source
	}

	/// Is the lexer at the last token
	pub fn at_end(&self) -> bool {
		self.offset.0 as usize == self.source.len()
	}

	/// Current position in file
	#[inline(always)]
	pub fn offset(&self) -> SourceOffset {
		self.offset
	}

	#[inline(always)]
	pub fn checkpoint(&self) -> Cursor {
		Cursor::new(self.offset(), self.token)
	}

	/// Rewinds the lexer back to the given checkpoint
	pub fn rewind(&mut self, cursor: Cursor) {
		debug_assert!(cursor.offset() <= self.offset());
		self.offset = cursor.offset();
		self.token = cursor.token();
	}

	/// Advances the lexer to the end of the given token
	pub fn hop(&mut self, cursor: Cursor) {
		debug_assert!(cursor.offset().0 as usize >= (self.offset.0 + self.token.len()) as usize);
		self.offset = cursor.offset();
		self.token = cursor.token();
	}

	/// Moves the lexer one token forward, returning that token
	pub fn advance(&mut self) -> Token {
		self.token = self.read_next_token(self.offset.0);
		self.offset.0 += self.token.len();
		self.token
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Feature>(), 1);
	assert_eq!(::std::mem::size_of::<Lexer>(), 32);
}
