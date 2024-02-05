pub use std::fmt::{Result, Write};
use std::ops::Deref;

use hdx_atom::Atom;
use hdx_parser::Spanned;
use oxc_allocator::Box;

pub trait WriteCss<'a>: Sized {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result;
}

pub trait CssWriter {
	fn write_newline(&mut self) -> Result;
	fn write_str(&mut self, str: &str) -> Result;
	fn write_char(&mut self, char: char) -> Result;
	fn write_trivia_str(&mut self, str: &str) -> Result;
	fn write_trivia_char(&mut self, char: char) -> Result;
	fn write_indent(&mut self) -> Result;
	fn indent(&mut self);
	fn dedent(&mut self);
}

pub struct BaseCssWriter<W>
where
	W: Write,
{
	sink: W,
	col: u32,
	line: u32,
	indent: u8,
	compressed: bool,
}

impl<W> BaseCssWriter<W>
where
	W: Write,
{
	pub fn new(sink: W, compressed: bool) -> Self {
		BaseCssWriter { sink, col: 0, line: 0, indent: 0, compressed }
	}
}

impl<W> CssWriter for BaseCssWriter<W>
where
	W: Write,
{
	fn write_newline(&mut self) -> Result {
		if !self.compressed {
			self.write_char('\n')?;
			self.line += 1;
		}
		Ok(())
	}

	fn write_trivia_str(&mut self, str: &str) -> Result {
		if !self.compressed {
			self.write_str(str)?;
		}
		Ok(())
	}

	fn write_str(&mut self, str: &str) -> Result {
		self.col += str.len() as u32;
		self.sink.write_str(str)
	}

	fn write_trivia_char(&mut self, ch: char) -> Result {
		if !self.compressed {
			self.write_char(ch)?;
		}
		Ok(())
	}

	fn write_char(&mut self, ch: char) -> Result {
		if ch == '\n' {
			self.line += 1
		} else {
			self.col += 1
		}
		self.sink.write_char(ch)
	}

	fn write_indent(&mut self) -> Result {
		if !self.compressed {
			for _ in 0..(self.indent) {
				self.write_char('\t')?;
			}
			self.col += self.indent as u32;
		}
		Ok(())
	}

	fn indent(&mut self) {
		if !self.compressed {
			self.indent += 1
		}
	}

	fn dedent(&mut self) {
		if !self.compressed {
			self.indent -= 1
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Option<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(value) = self { value.write_css(sink) } else { Ok(()) }
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Box<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		self.deref().write_css(sink)
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Spanned<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		self.node.write_css(sink)
	}
}

impl<'a> WriteCss<'a> for Atom {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str(self.as_ref())
	}
}

impl<'a> WriteCss<'a> for f32 {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str(self.to_string().as_str())
	}
}

impl<'a> WriteCss<'a> for i32 {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str(self.to_string().as_str())
	}
}

impl<'a> WriteCss<'a> for char {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_char(*self)
	}
}
