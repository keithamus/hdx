use std::{fmt::Display, hash::Hash, ops::Add};

use crate::SourceOffset;
use miette::SourceSpan;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
	pub start: SourceOffset,
	pub end: SourceOffset,
}

impl Span {
	#[inline]
	pub const fn new(start: SourceOffset, end: SourceOffset) -> Self {
		debug_assert!(start.0 <= end.0);
		Self { start, end }
	}

	#[inline]
	pub fn end(self, end: SourceOffset) -> Self {
		debug_assert!(self.start <= end);
		Self { start: self.start, end }
	}

	pub fn dummy() -> Self {
		Self::new(SourceOffset::DUMMY, SourceOffset::DUMMY)
	}

	pub fn is_dummy(&self) -> bool {
		self.start == self.end && self.end == SourceOffset::DUMMY
	}

	pub fn size(&self) -> u32 {
		debug_assert!(self.start <= self.end);
		self.end.0 - self.start.0
	}

	pub fn span_contents(self, source: &'_ str) -> SpanContents<'_> {
		SpanContents::new(self, source)
	}
}

impl Add for Span {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		let start = if self.start < rhs.start { self.start } else { rhs.start };
		let end = if self.end > rhs.end { self.end } else { rhs.end };
		Self { start, end }
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SpanContents<'a> {
	span: Span,
	source: &'a str,
}

impl<'a> SpanContents<'a> {
	pub fn new(span: Span, source: &'a str) -> SpanContents<'a> {
		SpanContents { span, source }
	}

	pub fn line_and_column(&self) -> (u32, u32) {
		let mut line = 0;
		let mut column = 0;
		let mut offset = self.span.start.0;
		for char in self.source.chars() {
			if offset == 0 {
				break;
			}
			if char == '\n' {
				column = 0;
				line += 1;
			} else {
				column += 1;
			}
			offset -= char.len_utf8() as u32;
		}
		(line, column)
	}

	pub fn contents(&self) -> &'a str {
		&self.source[self.span.start.0 as usize..self.span.end.0 as usize]
	}

	pub fn size(&self) -> u32 {
		self.span.size()
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}..{})", self.start.0, self.end.0)
	}
}

impl From<Span> for SourceSpan {
	fn from(val: Span) -> Self {
		Self::new(miette::SourceOffset::from(val.start.0 as usize), val.size() as usize)
	}
}
