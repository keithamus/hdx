use core::fmt;

use crate::{CursorStream, Error, ToCursors};
use bumpalo::Bump;
use hdx_lexer::Cursor;

#[derive(Debug)]
pub struct ParserReturn<'a, T>
where
	T: ToCursors<'a>,
{
	pub output: Option<T>,
	pub source_text: &'a str,
	pub errors: Vec<Error>,
	pub trivia: Vec<Cursor>,
	with_trivia: bool,
}

impl<'a, T: ToCursors<'a>> ParserReturn<'a, T> {
	pub fn new(output: Option<T>, source_text: &'a str, errors: Vec<Error>, trivia: Vec<Cursor>) -> Self {
		Self { output, source_text, errors, trivia, with_trivia: false }
	}

	pub fn with_trivia(mut self) -> Self {
		self.with_trivia = true;
		self
	}

	pub fn write(&self, allocator: &'a Bump, f: &mut impl fmt::Write) -> fmt::Result {
		if let Some(out) = &self.output {
			let mut stream = CursorStream::new(allocator);
			ToCursors::to_cursors(out, &mut stream);
			stream.write(self.source_text, f)?;
		}
		Ok(())
	}
}

impl<'a, T: ToCursors<'a>> ToCursors<'a> for ParserReturn<'a, T> {
	fn to_cursors(&self, s: &mut crate::CursorStream<'a>) {
		if let Some(output) = &self.output {
			ToCursors::to_cursors(output, s);
		}
	}
}
