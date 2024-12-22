use bumpalo::{collections::Vec, Bump};
use css_lexer::{Cursor, Token};

use crate::CursorSink;

const SEPARATOR: Cursor = Cursor::dummy(Token::SPACE);

/// This is a very basic [CursorSink], that simply collects all tokens into a [Vec]. Each time [CursorSink::append()]
/// is called the given [Cursor] will be pushed into the [Vec]. Useful for doing other manipulation of the Cursors
/// if they need to be buffered for some reason.
///
/// It has some very minor smarts: it will insert a whitespace [Token] where two adjacent tokens require some separator,
/// but does not perform any other interesting operations on the cursors (e.g. minification).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorVecSink<'a> {
	cursors: Vec<'a, Cursor>,
}

impl<'a> CursorVecSink<'a> {
	pub fn new(allocator: &'a Bump) -> Self {
		Self { cursors: Vec::new_in(allocator) }
	}
}

impl CursorSink for CursorVecSink<'_> {
	fn append(&mut self, c: Cursor) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.cursors.last() {
			if last.token().needs_separator_for(c.into()) {
				self.cursors.push(SEPARATOR);
			}
		}
		self.cursors.push(c);
	}
}
