use core::fmt;

use crate::TokenHighlighter;
use css_lexer::{Cursor, Token};
use css_parse::CursorSink;

pub(crate) struct HTMLHighlightCursorStream<'a, T> {
	source_text: &'a str,
	writer: T,
	last_token: Option<Token>,
	err: Option<fmt::Error>,
	pub highlighter: TokenHighlighter,
}

impl<'a, T: fmt::Write> HTMLHighlightCursorStream<'a, T> {
	pub fn new(source_text: &'a str, writer: T) -> Self {
		Self { source_text, writer, last_token: None, err: None, highlighter: TokenHighlighter::new() }
	}

	pub fn finish(&mut self) {
		if self.err.is_none() {
			if let Err(err) = self.writer.write_str(
				r#"
		</code>
	</pre>
</body>"#,
			) {
				self.err = Some(err);
			}
		}
	}
}

impl<'a, T: fmt::Write> CursorSink for HTMLHighlightCursorStream<'a, T> {
	fn append(&mut self, c: Cursor) {
		if self.last_token.is_none() {
			if let Err(err) = self.writer.write_str(
				r#"
<!DOCTYPE html>
<head>
	<style>
		:root { background: #22272E; color: hotpink }
		.Tag { color: #8ddb8c }
		.Punctuation { color: #d1d7e0 }
		.Property { color: #6cb6ff }
		.PseudoClass { color: #6cb6ff }

		.unknown { color: grey }
		.deprecated { text-decoration: line-through }
		.experimental { text-decoration: wavy underline #bf4b8a 0.5px }
	</style>
</head>
<body>
	<pre>
		<code>
			"#,
			) {
				self.err = Some(err);
			}
		}
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
		if self.err.is_some() {
			return;
		}
		self.last_token = Some(c.into());
		let highlight = self.highlighter.get(c.into());
		if let Some(highlight) = highlight {
			if let Err(err) =
				self.writer.write_str(format!(r#"<span class="{}{}">"#, highlight.kind, highlight.modifier).as_str())
			{
				self.err = Some(err);
			}
		}
		if let Err(err) = c.write_str(self.source_text, &mut self.writer) {
			self.err = Some(err);
		}
		if highlight.is_some() {
			if let Err(err) = self.writer.write_str(r#"</span>"#) {
				self.err = Some(err);
			}
		}
	}
}

macro_rules! assert_highlight {
	($name: literal, $str: literal $(,)*) => {
		use bumpalo::{collections::String, Bump};
		use css_ast::{StyleSheet, Visitable};
		use css_parse::{Parser, ToCursors};

		let bump = Bump::default();
		let mut parser = Parser::new(&bump, $str);
		let result = parser.parse_entirely::<StyleSheet>();
		if !result.errors.is_empty() {
			panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), $str, result.errors[0]);
		}
		let mut actual = String::new_in(&bump);
		let mut cursors = HTMLHighlightCursorStream::new($str, &mut actual);
		let node = result.output.clone().unwrap();
		node.accept(&mut cursors.highlighter);
		result.to_cursors(&mut cursors);
		cursors.finish();
		::insta::assert_snapshot!($name, actual)
	};
}
pub(crate) use assert_highlight;
