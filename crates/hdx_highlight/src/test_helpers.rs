use core::fmt;

use crate::TokenHighlighter;
use bumpalo::{collections::Vec, Bump};
use hdx_lexer::Cursor;
use hdx_parser::CursorSink;

pub(crate) struct HTMLHighlightCursorStream<'a> {
	cursors: Vec<'a, Cursor>,
	pub highlighter: TokenHighlighter,
}

impl<'a> HTMLHighlightCursorStream<'a> {
	pub fn new(allocator: &'a Bump) -> Self {
		Self { cursors: Vec::new_in(allocator), highlighter: TokenHighlighter::new() }
	}

	pub fn write(&self, source: &str, f: &mut impl fmt::Write) -> fmt::Result {
		f.write_str(
			r#"
<!DOCTYPE html>
<head>
	<style>
		:root { background: #22272E; color: hotpink }
		.tag { color: #8ddb8c }
		.punctuation { color: #d1d7e0 }
		.property { color: #6cb6ff }
		.pseudo-class { color: #6cb6ff }

		.unknown { color: grey }
		.deprecated { text-decoration: line-through }
		.experimental { text-decoration: wavy underline #bf4b8a 0.5px }
	</style>
</head>
<body>
	<pre>
		<code>
		"#,
		)?;
		for c in self.iter_cursors() {
			let highlight = self.highlighter.get(c.into());
			if let Some(highlight) = highlight {
				f.write_str(format!(r#"<span class="{}{}">"#, highlight.kind, highlight.modifier).as_str())?;
			}
			c.write_str(source, f)?;
			if highlight.is_some() {
				f.write_str(r#"</span>"#)?;
			}
		}
		f.write_str(
			r#"
					</code>
				</pre>
			</body>"#,
		)?;
		Ok(())
	}
}

impl<'a> CursorSink for HTMLHighlightCursorStream<'a> {
	fn append(&mut self, c: Cursor) {
		self.cursors.push(c);
	}
	fn iter_cursors(&self) -> impl Iterator<Item = &Cursor> {
		self.cursors.iter()
	}
}

macro_rules! assert_highlight {
	($name: literal, $str: literal $(,)*) => {
		use bumpalo::{collections::String, Bump};
		use hdx_ast::css::{visit::Visitable, StyleSheet};
		use hdx_parser::{Features, Parser, ToCursors};

		let bump = Bump::default();
		let mut parser = Parser::new(&bump, $str, Features::default());
		let result = parser.parse_entirely::<StyleSheet>();
		if !result.errors.is_empty() {
			panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), $str, result.errors[0]);
		}
		let mut actual = String::new_in(&bump);
		let mut cursors = HTMLHighlightCursorStream::new(&bump);
		let node = result.output.clone().unwrap();
		dbg!(&node);
		node.accept(&mut cursors.highlighter);
		result.to_cursors(&mut cursors);
		cursors.write($str, &mut actual).unwrap();
		::insta::assert_snapshot!($name, actual)
	};
}
pub(crate) use assert_highlight;
