#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Box, Spanned};

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Shorthand<'a, T> {
	#[default]
	Implicit,
	Explicit(Box<'a, Spanned<T>>),
}

impl<'a, T> Shorthand<'a, T> {
	#[inline]
	pub fn is_implicit(&self) -> bool {
		matches!(self, Self::Implicit)
	}

	#[inline]
	pub fn is_explicit(&self) -> bool {
		!self.is_implicit()
	}
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct BoxShorthand<'a, T> {
	pub top: Shorthand<'a, T>,
	pub right: Shorthand<'a, T>,
	pub bottom: Shorthand<'a, T>,
	pub left: Shorthand<'a, T>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct XYShorthand<'a, T> {
	pub x: Shorthand<'a, T>,
	pub y: Shorthand<'a, T>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct DoubleShorthand<'a, T>(pub Shorthand<'a, T>, pub Shorthand<'a, T>);
