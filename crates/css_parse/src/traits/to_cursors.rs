use css_lexer::Cursor;

use crate::CursorSink;

/// This trait allows AST nodes to decompose themselves back into a set of (ordered) [Cursors][Cursor].
///
/// This trait is useful to implement because downstream operations can use it to reconstruct source text from Nodes,
/// including after mutating Nodes, such as transforming them (e.g. minification or formatting).
///
/// Nodes that implement this trait should call `s.append()` in the order that those [Cursors][Cursor] were parsed,
/// unless there's a good reason not to. Some good reasons not to:
///
///  - The specification supplies a specific grammar order.
///  - Doing so would require creating many intermediary enums or structs.
///
pub trait ToCursors {
	fn to_cursors(&self, s: &mut impl CursorSink);
}

impl<T> ToCursors for T
where
	T: Into<Cursor> + Clone,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.clone().into())
	}
}
