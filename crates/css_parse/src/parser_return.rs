use crate::{CursorSink, Error, ToCursors};
use css_lexer::Cursor;

#[derive(Debug)]
pub struct ParserReturn<'a, T>
where
	T: ToCursors,
{
	pub output: Option<T>,
	pub source_text: &'a str,
	pub errors: Vec<Error>,
	pub trivia: Vec<Cursor>,
	with_trivia: bool,
}

impl<'a, T: ToCursors> ParserReturn<'a, T> {
	pub fn new(output: Option<T>, source_text: &'a str, errors: Vec<Error>, trivia: Vec<Cursor>) -> Self {
		Self { output, source_text, errors, trivia, with_trivia: false }
	}

	pub fn with_trivia(mut self) -> Self {
		self.with_trivia = true;
		self
	}
}

impl<T: ToCursors> ToCursors for ParserReturn<'_, T> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(output) = &self.output {
			let sink = if self.with_trivia { todo!() } else { s };
			ToCursors::to_cursors(output, sink);
		}
	}
}
