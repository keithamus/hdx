#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	atom,
	css::{
		properties::Property,
		rules::{page::PageRule, Charset},
		selector::Selector,
		unknown::{UnknownAtRule, UnknownRule},
	},
	Atom, Atomizable, Box, Spanned, Vec,
};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "camelCase"))]
pub struct Stylesheet<'a> {
	pub rules: Vec<'a, StylesheetRule<'a>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum StylesheetRule<'a> {
	At(Box<'a, Spanned<AtRule<'a>>>),
	Style(Box<'a, Spanned<StyleRule<'a>>>),
	Unknown(Box<'a, Spanned<UnknownRule<'a>>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AtRule<'a> {
	Charset(Box<'a, Spanned<Charset>>),
	Page(Box<'a, Spanned<PageRule<'a>>>),
	Unknown(Box<'a, Spanned<UnknownAtRule<'a>>>),
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset, // atom!("charset")
	Page,    // atom!("page")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StyleRule<'a> {
	pub selectors: Box<'a, Spanned<SelectorSet<'a>>>,
	pub properties: Box<'a, Vec<'a, Spanned<Property<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<StyleRule<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SelectorSet<'a> {
	pub children: Vec<'a, Spanned<Selector<'a>>>,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Stylesheet>(), 32);
		assert_eq!(size_of::<StylesheetRule>(), 16);
		assert_eq!(size_of::<AtRule>(), 16);
		assert_eq!(size_of::<AtRuleId>(), 1);
		assert_eq!(size_of::<StyleRule>(), 24);
		assert_eq!(size_of::<SelectorSet>(), 32);
	}
}
