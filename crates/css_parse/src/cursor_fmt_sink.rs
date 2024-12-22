use core::fmt;

use css_lexer::{Cursor, Token};

use crate::CursorSink;

/// This is a [CursorSink] that wraps a Writer (`impl fmt::Write`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given Writer - using the given `&'a str` as the original source.
/// This is useful as way to turn Cursors into Strings or [u8]s (or files or whatever else implements [fmt::Write]).
pub struct CursorFmtSink<'a, T: fmt::Write> {
	source_text: &'a str,
	writer: T,
	last_token: Option<Token>,
	err: Option<fmt::Error>,
}

impl<'a, T: fmt::Write> CursorFmtSink<'a, T> {
	pub fn new(source_text: &'a str, writer: T) -> Self {
		Self { source_text, writer, last_token: None, err: None }
	}
}

impl<'a, T: fmt::Write> CursorSink for CursorFmtSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		if self.err.is_some() {
			return;
		}
		if let Some(last) = self.last_token {
			if last.needs_separator_for(c.into()) {
				if let Err(err) = self.writer.write_char(' ') {
					self.err = Some(err);
				}
			}
		}
		self.last_token = Some(c.into());
		if self.err.is_some() {
			return;
		}
		if let Err(err) = c.write_str(self.source_text, &mut self.writer) {
			self.err = Some(err);
		}
	}
}
