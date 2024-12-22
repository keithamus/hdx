use css_lexer::Cursor;

/// This trait provides the generic `impl` that [ToCursors][crate::ToCursors] can use. This provides just enough API
/// surface for nodes to put the cursors they represent into some buffer which can later be read, the details of which
/// are elided.
pub trait CursorSink {
	fn append(&mut self, c: Cursor);
}
