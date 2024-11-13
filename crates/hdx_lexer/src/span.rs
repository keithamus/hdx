use std::{fmt::Display, hash::Hash};

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

	pub fn source_text<'a>(&self, source_text: &'a str) -> &'a str {
		&source_text[self.start.0 as usize..self.end.0 as usize]
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
