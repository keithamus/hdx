use std::{fmt::Display, hash::Hash};

use hdx_atom::{Atom, Atomizable};
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

	#[inline]
	pub fn end(self, end: u32) -> Self {
		Self { start: self.start, end }
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
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}..{})", self.start, self.end)
	}
}

impl From<Span> for SourceSpan {
	fn from(val: Span) -> Self {
		Self::new(SourceOffset::from(val.start as usize), val.size() as usize)
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Spanned<T> {
	pub node: T,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub span: Span,
}

impl<T> Spanned<T> {
	pub fn dummy(node: T) -> Self {
		Self { node, span: Span::dummy() }
	}
}

impl<T: Atomizable> Atomizable for Spanned<T> {
	fn from_atom(atom: Atom) -> Option<Self> {
		T::from_atom(atom).map(|node| Self { node, span: Span::dummy() })
	}

	fn to_atom(&self) -> Atom {
		self.node.to_atom()
	}
}

impl<T: Default> Default for Spanned<T> {
	fn default() -> Self {
		Self::dummy(T::default())
	}
}
