#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	atom,
	css::{
		properties::Property,
		rules::{page::CSSPageRule, CSSCharsetRule},
		selector::Selector,
		unknown::{UnknownAtRule, UnknownRule},
	},
	Atom, Atomizable, Box, Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSStyleSheet<'a> {
	pub rules: Vec<'a, CSSRule<'a>>,
}

// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum CSSRule<'a> {
	Charset(Box<'a, Spanned<CSSCharsetRule>>),
	Page(Box<'a, Spanned<CSSPageRule<'a>>>),
	Style(Box<'a, Spanned<CSSStyleRule<'a>>>),
	UnknownAt(Box<'a, Spanned<UnknownAtRule<'a>>>),
	Unknown(Box<'a, Spanned<UnknownRule<'a>>>),
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset, // atom!("charset")
	Page,    // atom!("page")
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSStyleRule<'a> {
	pub selectors: Box<'a, Spanned<SelectorSet<'a>>>,
	pub declarations: Box<'a, Vec<'a, Spanned<Property<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<CSSStyleRule<'a>>>>,
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
		assert_eq!(size_of::<CSSStyleSheet>(), 32);
		assert_eq!(size_of::<CSSRule>(), 16);
		assert_eq!(size_of::<AtRuleId>(), 1);
		assert_eq!(size_of::<CSSStyleRule>(), 24);
		assert_eq!(size_of::<SelectorSet>(), 32);
	}
}
