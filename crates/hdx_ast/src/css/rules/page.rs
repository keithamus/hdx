use hdx_lexer::Span;
use oxc_allocator::{Box, Vec};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	atom, css::properties::Property, Atom, Atomizable, Spanned, Specificity, ToSpecificity,
};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageRule<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub selectors: Box<'a, Spanned<PageSelectorList<'a>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Box<'a, Vec<'a, Spanned<Property<'a>>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Box<'a, Vec<'a, Spanned<PageMarginRule<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelectorList<'a> {
	pub children: Vec<'a, Spanned<PageSelector<'a>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelector<'a> {
	pub page_type: Option<Atom>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub pseudos: Vec<'a, Spanned<PagePseudoClass>>,
}

impl<'a> PageSelector<'a> {
	pub fn selector(&self) -> &str {
		todo!();
		// format!("{}{}", self.page_type.unwrap_or("").to_owned(), self.pseudos.into_iter().fold("", |p| p.as_str())join("")).as_str()
	}

	pub fn specificity(&self) -> Specificity {
		let mut spec = Specificity(self.page_type.is_some() as u8, 0, 0);
		for pseudo in &self.pseudos {
			spec += pseudo.specificity();
		}
		spec
	}
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "lowercase"))]
pub enum PagePseudoClass {
	Left,
	Right,
	First,
	Blank,
}

impl ToSpecificity for PagePseudoClass {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Blank => Specificity(0, 1, 0),
			Self::First => Specificity(0, 1, 0),
			Self::Left => Specificity(0, 0, 1),
			Self::Right => Specificity(0, 0, 1),
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageMarginRule<'a> {
	pub margin_box: PageMarginBox,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Spanned<Property<'a>>>,
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "kebab-case"))]
pub enum PageMarginBox {
	TopLeftCorner,     // atom!("top-left-corner")
	TopLeft,           // atom!("top-left")
	TopCenter,         // atom!("top-center")
	TopRight,          // atom!("top-right")
	TopRightCorner,    // atom!("top-right-corner")
	RightTop,          // atom!("right-top")
	RightMiddle,       // atom!("right-middle")
	RightBottom,       // atom!("right-bottom")
	BottomRightCorner, // atom!("bottom-right-corner")
	BottomRight,       // atom!("bottom-right")
	BottomCenter,      // atom!("bottom-center")
	BottomLeft,        // atom!("bottom-left")
	BottomLeftCorner,  // atom!("bottom-left-corner")
	LeftBottom,        // atom!("left-bottom")
	LeftMiddle,        // atom!("left-middle")
	LeftTop,           // atom!("left-top")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<PagePseudoClass>(), 1);
		assert_eq!(size_of::<PageMarginBox>(), 1);
		assert_eq!(size_of::<PageMarginRule>(), 40);
		assert_eq!(size_of::<PagePseudoClass>(), 1);
	}

	#[test]
	fn test_specificity() {
		assert_eq!(PagePseudoClass::Left.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::Right.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::First.specificity(), Specificity(0, 1, 0));
		assert_eq!(PagePseudoClass::Blank.specificity(), Specificity(0, 1, 0));
	}
}
