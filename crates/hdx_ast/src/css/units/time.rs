use hdx_atom::{atom, Atom};
use hdx_derive::Writable;
use hdx_lexer::Token;
use hdx_parser::FromToken;

use super::{AbsoluteUnit, CSSFloat};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Time {
	#[writable(suffix = "ms")]
	Ms(CSSFloat),
	#[writable(suffix = "s")]
	S(CSSFloat),
}

impl Time {
	pub fn new(val: CSSFloat, unit: Atom) -> Option<Self> {
		match unit {
			atom!("ms") => Some(Self::Ms(val.into())),
			atom!("s") => Some(Self::S(val.into())),
			_ => None,
		}
	}
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

impl FromToken for Time {
	fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::Dimension(n, unit, _) => Self::new(n.into(), unit.clone()),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Time, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Time, "0s");
		// Truncates to 7dp
		assert_parse!(Time, "1.2345678901234s", "1.2345679s");
		// Removes redundant dp
		assert_parse!(Time, "-1.0s", "-1s");
	}
}
