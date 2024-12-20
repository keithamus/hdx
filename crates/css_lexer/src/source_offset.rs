use crate::{Cursor, Span, Token};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SourceOffset(pub u32);

impl SourceOffset {
	pub const DUMMY: SourceOffset = SourceOffset(u32::MAX);

	pub fn as_span(&self, t: Token) -> Span {
		Span::new(*self, Self(self.0 + t.len()))
	}

	pub fn as_cursor(&self, t: Token) -> Cursor {
		Cursor::new(*self, t)
	}
}

impl From<SourceOffset> for miette::SourceOffset {
	fn from(value: SourceOffset) -> Self {
		Self::from(value.0 as usize)
	}
}

impl PartialEq<u32> for SourceOffset {
	fn eq(&self, other: &u32) -> bool {
		self.0 == *other
	}
}

impl From<SourceOffset> for usize {
	fn from(value: SourceOffset) -> Self {
		value.0 as usize
	}
}
