#[cfg(feature = "serde")]
use serde::Serialize;

use super::{CounterStyle, Expr, Image, PredefinedCounterStyle, Shorthand};
use crate::{atom, Atom, Atomizable, Box, Spanned};

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

// https://drafts.csswg.org/css-lists-3/#funcdef-counters
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ListStylePositionValue {
	#[default]
	Outside, // atom!("outside")
	Inside, // atom!("inside")
}

// https://drafts.csswg.org/css-lists-3/#funcdef-counters
#[derive(Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ListStyleImageValue<'a> {
	#[default]
	None,
	Image(Spanned<Image<'a>>),
}
//
// https://drafts.csswg.org/css-lists-3/#funcdef-counters
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ListStyleTypeValue<'a> {
	CounterStyle(Spanned<CounterStyle<'a>>),
	String(Atom),
	None,
}

impl<'a> Default for ListStyleTypeValue<'a> {
	fn default() -> Self {
		Self::CounterStyle(Spanned::dummy(CounterStyle::Predefined(PredefinedCounterStyle::Disc)))
	}
}

// https://drafts.csswg.org/css-lists/#propdef-list-style
#[derive(Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct ListStyleShorthand<'a> {
	pub position: Shorthand<'a, Expr<'a, ListStylePositionValue>>,
	pub image: Shorthand<'a, Expr<'a, ListStyleImageValue<'a>>>,
	pub marker: Shorthand<'a, Expr<'a, ListStyleTypeValue<'a>>>,
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
		assert_eq!(size_of::<ListStyleTypeValue>(), 32);
		assert_eq!(size_of::<ListStyleImageValue>(), 24);
		assert_eq!(size_of::<ListStylePositionValue>(), 1);
	}
}
