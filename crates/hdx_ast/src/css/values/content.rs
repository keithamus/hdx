#[cfg(feature = "serde")]
use serde::Serialize;

use super::{CounterOrCounters, Image};
use crate::{atom, Atom, Atomizable, Spanned, Vec};

//
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ContentsValue<'a> {
	#[default]
	Normal,
	None,
	Replacement(Spanned<ContentReplacement<'a>>),
	List(Spanned<ContentList<'a>>),
}

//
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ContentAlt<'a> {
	#[default]
	None,
	String(Atom),
	Counter(Spanned<CounterOrCounters<'a>>),
}

//
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct ContentReplacement<'a> {
	pub image: Image<'a>,
	pub alt: ContentAlt<'a>,
}

//
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct ContentList<'a> {
	pub values: Vec<'a, ContentElement<'a>>,
	pub alt: Vec<'a, ContentAlt<'a>>,
}
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ContentElement<'a> {
	String(Atom),
	Contents,
	Image(Spanned<Image<'a>>),
	Counter(Spanned<CounterOrCounters<'a>>),
	Quote(Quote),
	Leader(Leader),
}

// https://drafts.csswg.org/css-content-3/#typedef-quote
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Quote {
	OpenQuote,    // atom!("open-quote")
	CloseQuote,   // atom!("close-quote")
	NoOpenQuote,  // atom!("no-open-quote")
	NoCloseQuote, // atom!("no-close-quote")
}

// https://drafts.csswg.org/css-content-3/#leader-function
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Leader {
	Dotted,
	Solid,
	Space,
	String(Atom),
}

// https://drafts.csswg.org/css-content-3/#leader-function
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum QuotesValue<'a> {
	#[default]
	Auto,
	None,
	Custom(Vec<'a, (Atom, Atom)>),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ContentsValue>(), 80);
		assert_eq!(size_of::<ContentAlt>(), 40);
		assert_eq!(size_of::<ContentReplacement>(), 56);
		assert_eq!(size_of::<ContentList>(), 64);
		assert_eq!(size_of::<ContentElement>(), 40);
		assert_eq!(size_of::<Quote>(), 1);
		assert_eq!(size_of::<Leader>(), 16);
		assert_eq!(size_of::<QuotesValue>(), 40);
	}
}
