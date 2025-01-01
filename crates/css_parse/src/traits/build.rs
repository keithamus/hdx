use css_lexer::Cursor;

use crate::Parser;

/// This trait allows AST nodes to construct themselves from a single Cursor from the [Parser].
///
/// AST nodes that implement this should be able to infallably construct themsevles from the given cursor. It's likely
/// they'll need to implement [Peek][crate::Peek] to complete the contract: any AST node returning `true` from
/// [Peek][crate::Peek] should be able to parse the first token, and given this is a single token Node,
/// [Peek][crate::Peek] effectively demonstrates it can construct itself completely, when true.
pub trait Build<'a>: Sized {
	fn build(p: &Parser<'a>, c: Cursor) -> Self;
}
