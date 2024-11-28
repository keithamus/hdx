use hdx_lexer::Cursor;

use crate::CursorStream;

pub trait ToCursors<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>);
}

impl<'a, T> ToCursors<'a> for T
where
	T: Into<Cursor> + Clone,
{
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.clone().into())
	}
}
