#[cfg(feature = "serde")]
use serde::Serialize;

use super::{ColorValue, Expr, MathExpr, Shorthand};
use crate::{atom, Atom, Atomizable, Box};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct TextDecorationShorthand<'a> {
	pub line: Shorthand<'a, Expr<'a, TextDecorationLineValue>>,
	pub style: Shorthand<'a, Expr<'a, TextDecorationStyleValue>>,
	pub color: Shorthand<'a, MathExpr<'a, ColorValue<'a>>>,
}

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextDecorationLineValue {
	#[default]
	None,
	Style {
		underline: bool,
		overline: bool,
		line_through: bool,
		blink: bool,
	},
}

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextDecorationSkipInkValue {
	#[default]
	Auto, // atom!("auto")
	None, // atom!("none")
	All,  // atom!("all")
}

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextDecorationStyleValue {
	#[default]
	Solid, // atom!("solid"),
	Double, // atom!("double")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Wavy,   // atom!("wavy")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<TextDecorationShorthand>(), 24);
		assert_eq!(size_of::<TextDecorationStyleValue>(), 1);
		assert_eq!(size_of::<TextDecorationSkipInkValue>(), 1);
	}
}
