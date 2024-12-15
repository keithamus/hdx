use core::fmt;

use crate::{CursorSink, Error, ToCursors};
use bumpalo::Bump;
use hdx_atom::Atomizable;
use hdx_lexer::{Cursor, DimensionUnit, Kind};

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

	pub fn write(&self, sink: &mut impl CursorSink, f: &mut impl fmt::Write) -> fmt::Result {
		if let Some(out) = &self.output {
			ToCursors::to_cursors(out, sink);
		}
		let mut last_kind: Kind = Kind::Eof;
		let mut cursors = sink.iter_cursors().peekable();
		while let Some(c) = cursors.next() {
			let t = c.token();
			let kind = t.kind();
			if last_kind.ambiguous_without_whitespace() {
				match kind {
					Kind::Number if !t.has_sign() => {
						f.write_char(' ')?;
					}
					// An ident with a trailing `(` is a Function token, always separate these with whitespace to ensure they're not
					// accidentally combined into a single function token.
					Kind::LeftParen if last_kind == Kind::Ident => f.write_char(' ')?,
					Kind::Dimension | Kind::Ident | Kind::Function | Kind::Url => f.write_char(' ')?,
					_ => {}
				}
			} else if last_kind == Kind::Ident && kind == Kind::LeftParen {
				f.write_char(' ')?;
			}
			if kind == Kind::Semicolon {
				if let Some(cursor) = cursors.peek() {
					if *cursor == Kind::RightParen {
						continue;
					}
				}
			}
			c.write_str(self.source_text, f)?;
			last_kind = kind;
		}
		Ok(())
	}
}

impl<'a, T: ToCursors> ToCursors for ParserReturn<'a, T> {
	fn to_cursors(&self, s: &mut impl crate::CursorSink) {
		if let Some(output) = &self.output {
			ToCursors::to_cursors(output, s);
		}
	}
}
