use bitmask_enum::bitmask;
use hdx_syntax::identifier::is_ident_str;
use smallvec::SmallVec;

pub use std::fmt::{Result, Write};

use hdx_atom::Atom;
use hdx_lexer::QuoteStyle;
use hdx_parser::{Spanned, Vec};

pub trait WriteCss<'a>: Sized {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result;
}

#[bitmask(u16)]
pub enum OutputOption {
	Nesting,
	Whitespace,
	Comments,
	Trailing,
	InconsistentQuotes,
	QuotedIdentLikeStrings,
	RedundantRules,
	RedundantDeclarations,
	RedundantShorthandValues,
}

pub trait CssWriter {
	fn write_newline(&mut self) -> Result;
	fn write_str(&mut self, str: &str) -> Result;
	fn write_char(&mut self, ch: char) -> Result;
	fn write_comment(&mut self, str: &str) -> Result;
	fn write_with_quotes(&mut self, str: &str, quote: QuoteStyle, could_be_ident: bool) -> Result;
	fn write_trailing_char(&mut self, ch: char) -> Result;
	fn write_whitespace(&mut self) -> Result;
	fn write_indent(&mut self) -> Result;
	fn indent(&mut self);
	fn dedent(&mut self);
	fn can_output(&self, opt: OutputOption) -> bool;
}

pub struct BaseCssWriter<W>
where
	W: Write,
{
	sink: W,
	col: u32,
	line: u32,
	indent: u8,
	opts: OutputOption,
}

impl<W> BaseCssWriter<W>
where
	W: Write,
{
	pub fn new(sink: W, opts: OutputOption) -> Self {
		BaseCssWriter { sink, col: 0, line: 0, indent: 0, opts }
	}
}

impl<W> CssWriter for BaseCssWriter<W>
where
	W: Write,
{
	#[inline]
	fn can_output(&self, opt: OutputOption) -> bool {
		self.opts.contains(opt)
	}

	#[inline]
	fn write_str(&mut self, str: &str) -> Result {
		self.col += str.len() as u32;
		self.sink.write_str(str)
	}

	#[inline]
	fn write_char(&mut self, ch: char) -> Result {
		if ch == '\n' {
			self.line += 1
		} else {
			self.col += 1
		}
		self.sink.write_char(ch)
	}

	#[inline]
	fn write_newline(&mut self) -> Result {
		if self.can_output(OutputOption::Whitespace) {
			self.write_char('\n')?;
		}
		Ok(())
	}

	#[inline]
	fn write_comment(&mut self, str: &str) -> Result {
		if self.can_output(OutputOption::Comments) {
			self.write_str(str)?;
		}
		Ok(())
	}
	
	#[inline]
	fn write_with_quotes(&mut self, str: &str, quote: QuoteStyle, could_be_ident: bool) -> Result {
		if could_be_ident && !self.can_output(OutputOption::QuotedIdentLikeStrings) && is_ident_str(str) {
			self.write_str(str)
		} else if !self.can_output(OutputOption::InconsistentQuotes) {
			QuoteStyle::Double.write_css(self)?;
			self.write_str(str)?;
			QuoteStyle::Double.write_css(self)
		} else {
			quote.write_css(self)?;
			self.write_str(str)?;
			quote.write_css(self)
		}
	}

	#[inline]
	fn write_trailing_char(&mut self, ch: char) -> Result {
		if self.can_output(OutputOption::Trailing) {
			self.write_char(ch)?;
		}
		Ok(())
	}

	#[inline]
	fn write_whitespace(&mut self) -> Result {
		if self.can_output(OutputOption::Whitespace) {
			self.write_char(' ')?;
		}
		Ok(())
	}

	#[inline]
	fn write_indent(&mut self) -> Result {
		if self.can_output(OutputOption::Whitespace) {
			for _ in 0..(self.indent) {
				self.write_char('\t')?;
			}
			self.col += self.indent as u32;
		}
		Ok(())
	}

	#[inline]
	fn indent(&mut self) {
		self.indent += 1
	}

	#[inline]
	fn dedent(&mut self) {
		self.indent -= 1
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Option<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(value) = self {
			value.write_css(sink)
		} else {
			Ok(())
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Vec<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		for item in self.iter() {
			item.write_css(sink)?;
		}
		Ok(())
	}
}

// TODO: const N: usize?
impl<'a, T: WriteCss<'a>> WriteCss<'a> for SmallVec<[T; 1]> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let mut iter = self.iter().peekable();
		while let Some(w) = iter.next() {
			w.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}
impl<'a, T: WriteCss<'a>> WriteCss<'a> for SmallVec<[T; 2]> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let mut iter = self.iter().peekable();
		while let Some(w) = iter.next() {
			w.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}


impl<'a> WriteCss<'a> for QuoteStyle {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Double => sink.write_char('"'),
			Self::Single => {
				if sink.can_output(OutputOption::InconsistentQuotes) {
					sink.write_char('\'')
				} else {
					sink.write_char('"')
				}
			}
			Self::None => Ok(()),
		}
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
