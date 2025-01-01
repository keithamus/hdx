use std::{fmt::Display, hash::Hash, ops::Add};

use crate::SourceOffset;

/// Represents a range of text within a document, as a Start and End offset.
///
/// Effectively two [SourceOffsets][SourceOffset] in one struct.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
	start: SourceOffset,
	end: SourceOffset,
}

impl Span {
	/// Represents a fake Span with [SourceOffset::DUMMY] as the start/end offsets.
	pub const DUMMY: Span = Span::new(SourceOffset::DUMMY, SourceOffset::DUMMY);

	/// Creates a new [Span] given a starting [SourceOffset] and an ending [SourceOffset].
	///
	/// Asserts: start <= end
	#[inline]
	pub const fn new(start: SourceOffset, end: SourceOffset) -> Self {
		debug_assert!(start.0 <= end.0);
		Self { start, end }
	}

	/// Gets the starting [SourceOffset].
	#[inline]
	pub fn start(&self) -> SourceOffset {
		self.start
	}

	/// Gets the ending [SourceOffset].
	#[inline]
	pub fn end(&self) -> SourceOffset {
		self.end
	}

	/// Extends this [Span] into a new one with the end altered to be [SourceOffset].
	///
	/// Asserts: start <= end
	#[inline]
	pub fn with_end(self, end: SourceOffset) -> Self {
		debug_assert!(self.start <= end);
		Self { start: self.start, end }
	}

	/// Checks if the given [Span] would fit entirely within this [Span].
	pub fn contains(&self, span: Span) -> bool {
		self.start <= span.start && span.end <= self.end
	}

	/// Checks if the [Span] has no length.
	pub fn is_empty(&self) -> bool {
		self.start.0 == self.end.0
	}

	/// Returns the length of the [Span].
	pub fn len(&self) -> u32 {
		debug_assert!(self.start <= self.end);
		self.end.0 - self.start.0
	}

	pub fn span_contents(self, source: &'_ str) -> SpanContents<'_> {
		SpanContents::new(self, source)
	}
}

/// Extends this [Span], ensuring that the resulting new [Span] is broader than both this and the given [Span].
/// In other words the resulting span will always [Span::contains()] both [Spans][Span].
impl Add for Span {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		let start = if self.start < rhs.start { self.start } else { rhs.start };
		let end = if self.end > rhs.end { self.end } else { rhs.end };
		Self { start, end }
	}
}

/// Represents a [Span], and a pointer to the `&str` - the underlying source text that the [Span] originates from.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SpanContents<'a> {
	span: Span,
	source: &'a str,
}

impl<'a> SpanContents<'a> {
	/// Create a new [SpanContents] with the given [Span] and `&str`. The lifetime of the [SpanContents] will be bound to
	/// that of the `&str`.
	pub const fn new(span: Span, source: &'a str) -> SpanContents<'a> {
		SpanContents { span, source }
	}

	/// Scans the `&str`, accumulating newlines and columns until reaching the start of the [Span], returning those two
	/// numbers. The fist [u32] will be the line number (0-indexed) that the [Span] resides on. The second [u32] will be
	/// the column; the number of characters (0-indexed) between the last newline and the start of the [Span].
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

	/// Returns a new [str] slice of just the contents that the [Span] contains.
	pub fn contents(&self) -> &'a str {
		&self.source[self.span.start.0 as usize..self.span.end.0 as usize]
	}

	/// Delegates to [Span::is_empty()].
	pub fn is_empty(&self) -> bool {
		self.span.is_empty()
	}

	/// Delegates to [Span::len()].
	pub fn len(&self) -> u32 {
		self.span.len()
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}..{})", self.start.0, self.end.0)
	}
}

#[cfg(feature = "miette")]
impl From<Span> for miette::SourceSpan {
	fn from(val: Span) -> Self {
		Self::new(miette::SourceOffset::from(val.start.0 as usize), val.len() as usize)
	}
}

/// A trait representing an object that can derive its own [Span]. This is very similar to `From<MyStuct> for Span`,
/// however `From<MyStruct> for Span` requires `Sized`, meaning it is not `dyn` compatible.
pub trait Spanned {
	fn to_span(&self) -> Span;
}
