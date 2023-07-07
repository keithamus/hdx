#[cfg(feature = "serde")]
use serde::Serialize;

use super::CounterStyle;
use crate::{Atom, Box};

// https://drafts.csswg.org/css-lists-3/#counter-functions
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CounterOrCounters<'a> {
	Counter(Counter<'a>),
	Counters(Counters<'a>),
}

// https://drafts.csswg.org/css-lists-3/#funcdef-counter
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Counter<'a> {
	pub name: Atom,
	pub style: Box<'a, CounterStyle<'a>>,
}

// https://drafts.csswg.org/css-lists-3/#funcdef-counters
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Counters<'a> {
	pub name: Atom,
	pub concatenator: Atom,
	pub style: Box<'a, CounterStyle<'a>>,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<CounterOrCounters>(), 24);
		assert_eq!(size_of::<Counter>(), 16);
		assert_eq!(size_of::<Counters>(), 24);
	}
}
