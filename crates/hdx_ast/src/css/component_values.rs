#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atom, Box, PairWise, Span, Spanned, Token, Vec};

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(Spanned<SimpleBlock<'a>>),
	Function(Spanned<Function<'a>>),
	Token(Token),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	pub pairwise: PairWise,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: Atom,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ComponentValue>(), 32);
		assert_eq!(size_of::<SimpleBlock>(), 16);
		assert_eq!(size_of::<Function>(), 16);
	}
}
