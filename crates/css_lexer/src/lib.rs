//! An implementation of the [CSS Syntax Level 3 tokenization algorithm][1]. It is intended as a low-level building
//! block for buidling parsers for CSS or CSS-alike languages (for example SASS).
//!
//! This crate provides the [Lexer] struct, which borrows `&str` and can incrementally produce [Tokens][Token]. The
//! encoding of the `&str` is assumed to be utf-8.
//!
//! The [Lexer] _may_ be configured with additional [Features][Feature] to allow for lexing tokens in ways which diverge
//! from the CSS specification (such as tokenizing comments using `//`). With no additional features this lexer is fully
//! spec compliant.
//!
//! [Tokens][Token] are _untyped_ (there are no super-classes like `Ident`); but they have a [Kind] which can be used to
//! determine their type. Tokens do not store the underlying character data, nor do they store their offsets. They just
//! provide "facts" about the underlying data. In order to re-build a string, each [Token] will need to be wrapped in a
//! [Cursor] and consult the original `&str` to get the character data. This design allows Tokens live in the stack,
//! avoiding heap allocation as they are always `size_of` `8`. Likewise [Cursors][Cursor] are always a `size_of` `12`.
//!
//! # Limitations
//!
//! The [Lexer] has limitations around document sizes and token sizes, in order to keep [Token], [SourceOffset] and
//! [Cursor] small. It's very unlikely the average document will run into these limitations, but they're listed here
//! for completeness:
//!
//! - Documents are limited to ~4gb in size. [SourceOffset] is a [u32] so cannot represent larger offsets. Attempting to
//! lex larger documents is considrered [undefined behaviour][2].
//!
//! - [Tokens][Token] are limited to ~4gb in length. A [Token's][Token] is a [u32] so cannot represent larger lengths.
//! If the lexer encounters a token with  larger length this is considered [undefined behaviour][2].
//!
//! - Number [Tokens][Token] are limited to 16,777,216 characters in length. For example encountering a number with
//! 17MM `0`s is considered [undefined behaviour][2]. This is not the same as the number value, which is an [f32].
//! (Please note that the CSS spec dictates numbers are f32, CSS does not have larger numbers).
//!
//! - Dimension [Tokens][Token] are limited to 4,096 numeric characters in length and 4,096 ident characters in length.
//! For example encountering a dimension with 4,097 `0`s is considered [undefined behaviour][2].
//!
//! # General usage
//!
//! A parser can be implemented on top of the [Lexer] by instantiating a [Lexer] with [Lexer::new()] or
//! [Lexer::new_with_features()] if you wish to opt-into non-spec-compliant features. The [Lexer] needs to be given a
//! `&str` which it will reference to produce Tokens.
//!
//! Repeatedly calling [Lexer::advance()] will move the Lexer's internal position one [Token] forward, and return the
//! newly lexed [Token], once the end of `&str` is reached [Lexer::advance()] will repeatedly return [Token::EOF].
//!
//! # Example
//!
//! ```
//! use css_lexer::*;
//! let mut lexer = Lexer::new("width: 1px");
//! assert_eq!(lexer.offset(), 0);
//! {
//! 	let token = lexer.advance();
//! 	assert_eq!(token, Kind::Ident);
//! 	let cursor = token.with_cursor(SourceOffset(0));
//! 	assert_eq!(cursor.str_slice(lexer.source()), "width");
//! }
//! {
//! 	let token = lexer.advance();
//! 	assert_eq!(token, Kind::Colon);
//! 	assert_eq!(token, ':');
//! }
//! {
//! 	let token = lexer.advance();
//! 	assert_eq!(token, Kind::Whitespace);
//! }
//! {
//! 	let token = lexer.advance();
//! 	assert_eq!(token, Kind::Dimension);
//! 	assert_eq!(token.dimension_unit(), DimensionUnit::Px);
//! }
//! ```
//!
//! [1]: https://drafts.csswg.org/css-syntax/#tokenization
//! [2]: https://en.wikipedia.org/wiki/Undefined_behavior

mod comment_style;
mod constants;
mod cursor;
mod dimension_unit;
mod feature;
mod kind;
mod kindset;
mod pairwise;
mod private;
mod quote_style;
mod source_offset;
mod span;
mod syntax;
mod token;
mod whitespace_style;

pub use comment_style::CommentStyle;
pub use cursor::Cursor;
pub use dimension_unit::DimensionUnit;
pub use feature::Feature;
pub use kind::Kind;
pub use kindset::KindSet;
pub use pairwise::PairWise;
pub use quote_style::QuoteStyle;
pub use source_offset::SourceOffset;
pub use span::{Span, SpanContents, Spanned};
pub use token::Token;
pub use whitespace_style::Whitespace;

/// The [Lexer] struct - the core of the library - borrows `&str` and can incrementally produce [Tokens][Token].
///
/// The encoding of the `&str` is assumed to be utf-8. Other sources should be re-encoded into utf-8 prior to ingesting
/// into the [Lexer].
///
/// The [Lexer] _may_ be configured with additional [Features][Feature] to allow for lexing tokens in ways which diverge
/// from the CSS specification (such as tokenizing comments using `//`). With no additional features this lexer is fully
/// spec compliant.
///
/// [Tokens][Token] are _untyped_ (there are no super-classes like `Ident`); but they have a [Kind] which can be used to
/// determine their type. Tokens do not store the underlying character data, nor do they store their offsets. They just
/// provide "facts" about the underlying data. In order to re-build a string, each [Token] will need to be wrapped in a
/// [Cursor] and consult the original `&str` to get the character data. This design allows Tokens live in the stack,
/// avoiding heap allocation as they are always `size_of` `8`. Likewise [Cursors][Cursor] are always a `size_of` `12`.
///
/// # Limitations
///
/// The [Lexer] has limitations around document sizes and token sizes, in order to keep [Token], [SourceOffset] and
/// [Cursor] small.
///
/// - Documents are limited to ~4gb in size. [SourceOffset] is a [u32] so cannot represent larger offsets. Attempting to
/// lex larger documents is considrered [undefined behaviour][2].
///
/// - [Tokens][Token] are limited to ~4gb in length. A [Token's][Token] is a [u32] so cannot represent larger lengths.
/// If the lexer encounters a token with  larger length this is considered [undefined behaviour][2].
///
/// - Number [Tokens][Token] are limited to 16,777,216 characters in length. For example encountering a number with
/// 17MM `0`s is considered [undefined behaviour][2]. This is not the same as the number value, which is an [f32].
/// (Please note that the CSS spec dictates numbers are f32, CSS does not have larger numbers).
///
/// - Dimension [Tokens][Token] are limited to 4,096 numeric characters in length and 4,096 ident characters in length.
/// For example encountering a dimension with 4,097 `0` is considered [undefined behaviour][2].
///
/// # General usage
///
/// A parser can be implemented on top of the [Lexer] by instantiating a [Lexer] with [Lexer::new()] or
/// [Lexer::new_with_features()] if you wish to opt-into non-spec-compliant features. The [Lexer] needs to be given a
/// `&str` which it will reference to produce Tokens.
///
/// Repeatedly calling [Lexer::advance()] will move the Lexer's internal position one [Token] forward, and return the
/// newly lexed [Token], once the end of `&str` is reached [Lexer::advance()] will repeatedly return [Token::EOF].
///
/// # Example
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("width: 1px");
/// assert_eq!(lexer.offset(), 0);
/// {
/// 	let token = lexer.advance();
/// 	assert_eq!(token, Kind::Ident);
/// 	let cursor = token.with_cursor(SourceOffset(0));
/// 	assert_eq!(cursor.str_slice(lexer.source()), "width");
/// }
/// {
/// 	let token = lexer.advance();
/// 	assert_eq!(token, Kind::Colon);
/// 	assert_eq!(token, ':');
/// }
/// {
/// 	let token = lexer.advance();
/// 	assert_eq!(token, Kind::Whitespace);
/// }
/// {
/// 	let token = lexer.advance();
/// 	assert_eq!(token, Kind::Dimension);
/// 	assert_eq!(token.dimension_unit(), DimensionUnit::Px);
/// }
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax/#tokenization
/// [2]: https://en.wikipedia.org/wiki/Undefined_behavior
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
	pub const fn offset(&self) -> SourceOffset {
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
	assert_eq!(::std::mem::size_of::<Lexer>(), 32);
}
