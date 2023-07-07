use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

/// Values
pub mod color; // https://drafts.csswg.org/css-color-5/#typedef-color
// TODO! pub mod image; // https://drafts.csswg.org/css-images-3/#typedef-image
pub mod angle;
pub mod backgrounds;
pub mod border_radius;
pub mod r#box;
pub mod compositing;
pub mod content;
pub mod counter_styles;
pub mod display;
pub mod expr;
pub mod fonts;
pub mod frequency;
pub mod images;
pub mod inline;
pub mod lists;
pub mod non_standard;
pub mod overflow;
pub mod page_floats;
pub mod position;
pub mod resolution;
pub mod shapes;
pub mod shorthand;
pub mod size_adjust;
pub mod sizing;
pub mod tables;
pub mod text;
pub mod text_decor;
pub mod time;
pub mod ui;
pub mod units;

pub use angle::*;
pub use backgrounds::*;
pub use border_radius::*;
pub use r#box::*;
pub use color::*;
pub use compositing::*;
pub use content::*;
pub use counter_styles::*;
pub use display::*;
pub use expr::*;
pub use fonts::*;
pub use frequency::*;
pub use images::*;
pub use inline::*;
pub use lists::*;
pub use non_standard::*;
pub use overflow::*;
pub use page_floats::*;
pub use position::*;
pub use resolution::*;
pub use shapes::*;
pub use shorthand::*;
pub use size_adjust::*;
pub use sizing::*;
pub use tables::*;
pub use text::*;
pub use text_decor::*;
pub use time::*;
pub use ui::*;
pub use units::*;

use crate::{atom, Atom, Atomizable, Box, Spanned};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ValueLike<'a> {
	Color(Box<'a, Spanned<Expr<'a, ColorValue<'a>>>>),
	Length(Box<'a, Spanned<MathExpr<'a, Length>>>),
	LengthPercentage(Box<'a, Spanned<MathExpr<'a, LengthPercentage>>>),
	FontFamily(Box<'a, Spanned<ExprList<'a, FontFamilyValue>>>),
	Unknown,
}

// https://drafts.csswg.org/css-values-4/#typedef-position
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PositionXY {
	pub x: HorizontalPosition,
	pub y: VerticalPosition,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum HorizontalPosition {
	Center,
	Length(LengthPercentage),
	Left(Option<LengthPercentage>),
	Right(Option<LengthPercentage>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum VerticalPosition {
	Center,
	Length(LengthPercentage),
	Top(Option<LengthPercentage>),
	Bottom(Option<LengthPercentage>),
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct NoNonGlobalValuesAllowed;

#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum AutoOrNone {
	#[default]
	Auto,
	None,
}

// https://drafts.csswg.org/css-values-4/#ratio-value
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Ratio(u8, u8);

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TimeOrAuto {
	#[default]
	Auto,
	Time(Time),
}

// https://drafts.csswg.org/css-values/#typedef-length-percentage
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum FrequencyPercentage {
	Frequency(Frequency),
	Percentage(f32),
	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
}

impl Hash for FrequencyPercentage {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Frequency(f) => f.hash(state),
			Self::Percentage(f) => f.to_bits().hash(state),
		}
	}
}

// https://drafts.csswg.org/css-values/#typedef-length-percentage
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AnglePercentage {
	Angle(Angle),
	Percentage(f32),
	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
}

impl Hash for AnglePercentage {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Angle(a) => a.hash(state),
			Self::Percentage(f) => f.to_bits().hash(state),
		}
	}
}

// https://drafts.csswg.org/css-values/#typedef-length-percentage
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum TimePercentage {
	Time(Time),
	Percentage(f32),
	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
}

impl Hash for TimePercentage {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Time(t) => t.hash(state),
			Self::Percentage(f) => f.to_bits().hash(state),
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<FrequencyPercentage>(), 8);
		assert_eq!(size_of::<AnglePercentage>(), 8);
		assert_eq!(size_of::<TimePercentage>(), 8);
		assert_eq!(size_of::<PositionXY>(), 24);
		assert_eq!(size_of::<HorizontalPosition>(), 12);
		assert_eq!(size_of::<VerticalPosition>(), 12);
	}
}
