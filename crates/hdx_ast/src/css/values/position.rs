#[cfg(feature = "serde")]
use serde::Serialize;

use super::{LengthPercentageOrAuto, MathExpr, Shorthand};
use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-position-3/#propdef-position
#[derive(Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum PositionValue {
	#[default]
	Static, // atom!("static")
	Relative, // atom!("relative")
	Absolute, // atom!("absolute")
	Sticky,   // atom!("sticky")
	Fixed,    // atom!("fixed")
}

// https://drafts.csswg.org/css-position-3/#inset-shorthands
#[derive(Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub struct InsetShorthand<'a> {
	pub top: Shorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>,
	pub right: Shorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>,
	pub bottom: Shorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>,
	pub left: Shorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>,
}
