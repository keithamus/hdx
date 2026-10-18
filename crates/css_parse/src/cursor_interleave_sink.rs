use crate::{Cursor, CursorSink, Kind, Vec};

/// This is a [CursorSink] that wraps a Sink (`impl CursorSink`) and a slice of [Cursor]s to interleave. On each
/// [CursorSink::append()] call, will append to the delegate sink, while also interleaving any of the Cursors from the
/// slice of [Cursor]s, in the right places.
///
/// This is useful as way to interleave ancilliary cursors, for example trivia.
pub struct CursorInterleaveSink<'a, S> {
	sink: &'a mut S,
	interleave: &'a [(Vec<'a, Cursor>, Cursor)],
	current_index: usize,
	#[cfg(debug_assertions)]
	seen_eof: bool,
}

impl<'a, S: CursorSink> CursorInterleaveSink<'a, S> {
	pub fn new(sink: &'a mut S, interleave: &'a [(Vec<'a, Cursor>, Cursor)]) -> Self {
		Self {
			sink,
			interleave,
			current_index: 0,
			#[cfg(debug_assertions)]
			seen_eof: false,
		}
	}
}

impl<'a, S: CursorSink> CursorSink for CursorInterleaveSink<'a, S> {
	fn append(&mut self, c: Cursor) {
		#[cfg(debug_assertions)]
		{
			debug_assert!(!self.seen_eof, "Received cursor after EOF: {:?}", c);
			if c == Kind::Eof {
				self.seen_eof = true;
			}
		}

		// Check if this content cursor has associated trivia
		while self.current_index < self.interleave.len() {
			let (trivia, associated_cursor) = &self.interleave[self.current_index];
			if associated_cursor.offset() == c.offset() && associated_cursor.token().kind() == c.token().kind() {
				for cursor in trivia {
					self.sink.append(*cursor);
				}
				self.current_index += 1;
				break;
			}
			if associated_cursor.offset() > c.offset() {
				break;
			}
			self.current_index += 1;
		}

		// If this is EOF, flush any remaining trivia before emitting EOF
		if c == Kind::Eof {
			while self.current_index < self.interleave.len() {
				let (trivia, _) = &self.interleave[self.current_index];
				for cursor in trivia {
					self.sink.append(*cursor);
				}
				self.current_index += 1;
			}
		}

		self.sink.append(c);
	}
}
