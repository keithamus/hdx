use bumpalo::{collections::Vec, Bump};
use hdx_lexer::Cursor;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorStream<'a> {
	cursors: Vec<'a, Cursor>,
}

impl<'a> CursorStream<'a> {
	pub fn new(allocator: &'a Bump) -> Self {
		Self { cursors: Vec::new_in(allocator) }
	}
}

impl<'a> CursorSink for CursorStream<'a> {
	fn append(&mut self, c: Cursor) {
		self.cursors.push(c);
	}
	fn iter_cursors(&self) -> impl Iterator<Item = &Cursor> {
		self.cursors.iter()
	}
}

pub trait CursorSink {
	fn append(&mut self, c: Cursor);
	fn iter_cursors(&self) -> impl Iterator<Item = &Cursor>;
}
