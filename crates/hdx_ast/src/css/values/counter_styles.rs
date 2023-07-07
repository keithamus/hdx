#[cfg(feature = "serde")]
use serde::Serialize;

use super::Image;
use crate::{atom, Atom, Atomizable, Box, Vec};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CounterStyle<'a> {
	Named(Atom),
	Symbols(Symbols<'a>),
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Symbols<'a> {
	kind: SymbolsType,
	symbols: Box<'a, Vec<'a, Symbol<'a>>>,
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Symbol<'a> {
	String(Atom),
	Image(Box<'a, Image<'a>>),
}

// https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SymbolsType {
	Cyclic,     // atom!("cyclic")
	Numeric,    // atom!("numeric")
	Alphabetic, // atom!("alphabetic")
	#[default]
	Symbolic, // atom!("symbolic")
	Fixed,      // atom!("fixed")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<CounterStyle>(), 16);
		assert_eq!(size_of::<Symbols>(), 16);
		assert_eq!(size_of::<Symbol>(), 16);
		assert_eq!(size_of::<SymbolsType>(), 1);
	}
}
