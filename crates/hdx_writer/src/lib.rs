pub(crate) use std::fmt::{Result, Write};

pub(crate) use hdx_ast::Spanned;
pub(crate) use hdx_atom::Atomizable;

mod css;

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
