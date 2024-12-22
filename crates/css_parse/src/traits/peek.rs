use css_lexer::{Cursor, KindSet};

use crate::Parser;

/// This trait allows AST nodes to indicate whether the [Parser] is in the right position to potentially
/// [Parse][crate::Parse] the node. Returning `true` from [Peek] is not a _guarantee_ that a node will successfully
/// parse, instead it offers an indication that the node can successfully parse the first node. This is useful for
/// cheaply comparing a set of Nodes to see which one might viably parse, rather than calling [Parser::try_parse()] on
/// each.
///
/// Nodes that implement this trait are entitled to peek any number of [Cursors][Cursor] ahead from the [Parser], to
/// determine if those [Cursors][Cursor] are viable to begin parsing, however there is a cost involved in peeking, so
/// it is worth being conservative; peek the minimum amount ahead to determine this. Most implementations can peek just
/// 1 [Cursor] ahead - this is provided as the second argument. To peek further, use the [Parser::peek_n()] method.
/// Calling `peek_n(2)` will return the [Cursor] after the provided one `peek_n(3)` will return the second [Cursor]
/// after, and so on.
///
/// For simple implementations it may be sufficient to just check the [Kind][css_lexer::Kind] of the given [Cursor].
/// Rather than implementing [Peek::peek()], supplying [Peek::PEEK_KINDSET] and relying on the provided [Peek::peek()]
/// method will work well.
///
/// However it is likely that more complex checks will be needed. In order to reason about the given [Cursor] (or other
/// cursors ahead) an implementation might want to call [Parser::parse_str()] - which takes a [Cursor] and returns the
/// underlying string to reason about. When comparing lots of strings, consider implementing a [phf::Map]. If comparing
/// just one string, consider [Parser::eq_ignore_ascii_case()] which can fail-fast, rather than parsing a whole string.
///
/// When peeking child nodes, implementations should _not_ call [Peek::Peek()] directly. Instead - call
/// [Parser::peek<T>()]. [Parser::parse_if_peek<T>()] also exists to conveniently parse a Node if it passes the peek
/// test.
///
/// If a Node can construct itself from a single [Cursor][css_lexer::Cursor] it should also implement
/// [Build][crate::Build], then it will get [Parse][crate::Parse] for free.
pub trait Peek<'a>: Sized {
	const PEEK_KINDSET: KindSet = KindSet::ANY;

	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Self::PEEK_KINDSET
	}
}
