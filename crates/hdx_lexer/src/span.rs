use std::{fmt::Display, hash::Hash};

use miette::{SourceOffset, SourceSpan};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Span {
	pub start: u32,
	pub end: u32,
}

impl Span {
	#[inline]
	pub const fn new(start: u32, end: u32) -> Self {
		Self { start, end }
	}

	pub fn dummy() -> Self {
		Self::new(u32::default(), u32::default())
	}

	pub fn is_dummy(&self) -> bool {
		self.start == self.end && self.end == u32::default()
	}

	pub fn size(&self) -> u32 {
		debug_assert!(self.start <= self.end);
		self.end - self.start
	}

	pub fn source_text<'a>(&self, source_text: &'a str) -> &'a str {
		&source_text[self.start as usize..self.end as usize]
	}

	pub fn with_start(&self, span: &Self) -> Self {
		Self { start: span.start, end: self.end }
	}

	pub fn with_end(&self, span: &Self) -> Self {
		Self { start: self.start, end: span.end }
	}

	pub fn up_to(&self, span: &Self) -> Self {
		Self { start: self.start, end: span.start }
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}..{})", self.start, self.end)
	}
}

impl From<Span> for SourceSpan {
	fn from(val: Span) -> Self {
		Self::new(SourceOffset::from(val.start as usize), SourceOffset::from(val.size() as usize))
	}
}
