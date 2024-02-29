use hdx_lexer::Token;
use hdx_parser::FromToken;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Value, Atomizable, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BaselineSource {
	#[default]
	Auto, // atom!("auto")
	First, // atom!("first")
	Last,  // atom!("last")
}

impl FromToken for BaselineSource {
    fn from_token(token: Token) -> Option<Self> {
		match token {
			Token::Ident(atom) => Self::from_atom(atom),
			_ => None
		}
    }
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<BaselineSource>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<BaselineSource>(&allocator, "auto", "auto");
		test_write::<BaselineSource>(&allocator, "first", "first");
		test_write::<BaselineSource>(&allocator, "last", "last");
	}
}
