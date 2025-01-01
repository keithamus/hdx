use crate::{Cursor, Span, Token};

/// Represents a position in the underlying source.
///
/// This is just a [u32] but wrapped in a newtype struct to help differentiate it from other [u32s][u32].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SourceOffset(pub u32);

impl SourceOffset {
	/// Represents a fake SourceOffset with [u32::MAX] as the number.
	pub const DUMMY: SourceOffset = SourceOffset(u32::MAX);

	/// Providing a [Token] can produce a [Span].
	pub fn as_span(&self, t: Token) -> Span {
		Span::new(*self, Self(self.0 + t.len()))
	}

	/// Providing a [Token] can produce a [Cursor], with the token embedded. This is a convenience wrapper around
	/// `Cursor::new(source_offset, token);`
	pub fn as_cursor(&self, t: Token) -> Cursor {
		Cursor::new(*self, t)
	}
}

#[cfg(feature = "miette")]
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
