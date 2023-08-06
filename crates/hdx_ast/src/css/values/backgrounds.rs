#[cfg(feature = "serde")]
use serde::Serialize;

use super::{ColorValue, Expr, Length, MathExpr, Shorthand};
use crate::{atom, Atom, Atomizable, Spanned};

// https://drafts.csswg.org/css-position-3/#inset-shorthands
#[derive(Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct BorderShorthand<'a> {
	pub line_width: Shorthand<'a, MathExpr<'a, LineWidth>>,
	pub line_style: Shorthand<'a, Expr<'a, LineStyle>>,
	pub color: Shorthand<'a, MathExpr<'a, ColorValue<'a>>>,
}

// https://drafts.csswg.org/css-backgrounds-3/#typedef-line-width
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LineWidth {
	Thin, // atom!("thin")
	#[default]
	Medium, // atom!("medium")
	Thick, // atom!("thick")
	Length(Spanned<Length>),
}

// https://drafts.csswg.org/css-backgrounds-3/#typedef-line-style
#[derive(Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum LineStyle {
	#[default]
	None, // atom!("none")
	Hidden, // atom!("hidden")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Solid,  // atom!("solid")
	Double, // atom!("double")
	Groove, // atom!("groove")
	Ridge,  // atom!("ridge")
	Inset,  // atom!("inset")
	Outset, // atom!("outset")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<BorderShorthand>(), 24);
		assert_eq!(size_of::<LineWidth>(), 16);
		assert_eq!(size_of::<LineStyle>(), 1);
	}
}
