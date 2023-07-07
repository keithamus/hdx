use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::{ColorValue, Length, LengthPercentage};
use crate::{atom, Atom, Atomizable, Box};

// https://drafts.csswg.org/css2/#value-def-absolute-size
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Image<'a> {
	Url(Atom),
	Gradient(Box<'a, Gradient<'a>>),
}

// https://drafts.csswg.org/css-images-3/#typedef-gradient
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Gradient<'a> {
	LinearGradient(LinearGradientDirection, Vec<ColorStopOrHint<'a>>),
	RepeatingLinearGradient(LinearGradientDirection, Vec<ColorStopOrHint<'a>>),
	RadialGradient(RadialGradientSize, RadialGradientEndingShape, Vec<ColorStopOrHint<'a>>),
	RepeatingRadialGradient(
		RadialGradientSize,
		RadialGradientEndingShape,
		Vec<ColorStopOrHint<'a>>,
	),
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-size
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum RadialGradientSize {
	#[default]
	ClosestCorner,
	ClosestSide,
	FarthestCorner,
	FarthestSide,
	Length(Length),
	LengthPercentage(LengthPercentage),
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-ending-shape
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum RadialGradientEndingShape {
	#[default]
	Circle, // atom!("circle")
	Ellipse, // atom!("ellipse")
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LinearGradientDirection {
	Angle(f32),
	ToLeft,
	ToRight,
	ToTop,
	ToBottom,
}

impl Hash for LinearGradientDirection {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Angle(f) => {
				0.hash(state);
				f.to_bits().hash(state);
			}
			Self::ToLeft => 1.hash(state),
			Self::ToRight => 2.hash(state),
			Self::ToTop => 3.hash(state),
			Self::ToBottom => 4.hash(state),
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ColorStopOrHint<'a> {
	ColorStop(Box<'a, ColorValue<'a>>, Option<LengthPercentage>),
	ColorHint(LengthPercentage),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Image>(), 16);
		assert_eq!(size_of::<Gradient>(), 40);
		assert_eq!(size_of::<LinearGradientDirection>(), 8);
		assert_eq!(size_of::<RadialGradientSize>(), 12);
		assert_eq!(size_of::<RadialGradientEndingShape>(), 1);
		assert_eq!(size_of::<ColorStopOrHint>(), 16);
	}
}
