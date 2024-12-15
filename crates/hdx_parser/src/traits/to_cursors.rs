use hdx_lexer::Cursor;

use crate::CursorSink;

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
