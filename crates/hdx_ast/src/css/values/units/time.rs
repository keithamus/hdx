#[cfg(feature = "serde")]
use serde::Serialize;

use super::{AbsoluteUnit, CSSFloat};
use crate::{Parsable, Writable};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Parsable, Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Time {
	#[writable(suffix = "ms")]
	#[parsable(Dimension)]
	Ms(CSSFloat),
	#[writable(suffix = "s")]
	#[parsable(Dimension)]
	S(CSSFloat),
}

impl Into<CSSFloat> for Time {
	fn into(self) -> CSSFloat {
		match self {
			Self::Ms(f) | Self::S(f) => f,
		}
	}
}

impl AbsoluteUnit for Time {
	fn to_base(&self) -> Self {
		Self::S(match self {
			Self::Ms(f) => *f / 1000.0,
			Self::S(f) => *f,
		})
	}
}
