#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::{component_values::ComponentValue, values::ValueLike},
	Atom, Box, Spanned, Vec,
};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownAtRule<'a> {
	pub name: Atom,
	pub prelude: Box<'a, Option<Spanned<UnknownPrelude<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<UnknownRule<'a>>>>,
	pub properties: Box<'a, Vec<'a, Spanned<UnknownDeclaration<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownRule<'a> {
	pub prelude: Box<'a, Option<Spanned<UnknownPrelude<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<UnknownRule<'a>>>>,
	pub properties: Box<'a, Vec<'a, Spanned<UnknownDeclaration<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct UnknownPrelude<'a> {
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownDeclaration<'a> {
	pub name: Atom,
	pub important: bool,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
	pub value_like: Spanned<ValueLike<'a>>,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<UnknownAtRule>(), 32);
		assert_eq!(size_of::<UnknownRule>(), 24);
		assert_eq!(size_of::<UnknownPrelude>(), 8);
		assert_eq!(size_of::<UnknownDeclaration>(), 48);
	}
}
