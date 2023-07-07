#[cfg(feature = "serde")]
use serde::Serialize;

use super::LengthPercentage;
use crate::{atom, Atom, Atomizable, Spanned};

// https://www.w3.org/TR/css-sizing-3/#propdef-box-sizing
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum BoxSizingValue {
	#[default]
	ContentBox, // atom!("content-box")
	BorderBox, // atom!("border-box")
}

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum Sizing {
	#[default]
	Auto,
	LengthPercentage(Spanned<LengthPercentage>),
	MinContent,
	MaxContent, // TODO: `intrinsic` non standard
	FitContentFunction(Spanned<LengthPercentage>),
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch, // TODO: -webkit-fill-available, -moz-available
	FitContent,
	Contain,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum MaxSizing {
	#[default]
	None,
	LengthPercentage(Spanned<LengthPercentage>),
	MinContent,
	MaxContent,
	FitContentFunction(Spanned<LengthPercentage>),
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,
	FitContent,
	Contain,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum RatioOrAuto {
	#[default]
	Auto,
	Ratio((u32, u32)),
}

// https://drafts.csswg.org/css-sizing-4/#intrinsic-contribution-override
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MinIntrinsicSizingValue {
	#[default]
	Legacy, // atom!("legacy")
	ZeroIfScroll,    // atom!("zero-if-scroll")
	ZeroIfExtrinsic, // atom!("zero-if-extrinsic")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Sizing>(), 20);
		assert_eq!(size_of::<MaxSizing>(), 20);
		assert_eq!(size_of::<RatioOrAuto>(), 12);
	}
}
