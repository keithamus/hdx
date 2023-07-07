#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Angle, MathExpr, PositiveLengthPercentage};
use crate::{atom, Atom, Atomizable, Spanned};

// https://drafts.csswg.org/css2/#value-def-absolute-size
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum AbsoluteSize {
	XxSmall, // atom!("xx-small")
	XSmall,  // atom!("x-small")
	Small,   // atom!("small")
	#[default]
	Medium, // atom!("medium")
	Large,   // atom!("large")
	XLarge,  // atom!("x-large")
	XxLarge, // atom!("xx-large")
}

// https://drafts.csswg.org/css2/#value-def-relative-size
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum RelativeSize {
	Larger,  // atom!("larger")
	Smaller, // atom!("smaller")
}

// https://drafts.csswg.org/css-fonts/#font-weight-prop
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontWeightValue {
	#[default]
	Normal,
	Bold,
	Bolder,
	Lighter,
	Number(u16),
}

// https://drafts.csswg.org/css-fonts-4/#propdef-font-size
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontSizeValue {
	Absolute(AbsoluteSize),
	Relative(RelativeSize),
	LengthPercentage(Spanned<PositiveLengthPercentage>),
	Math,
}

impl Default for FontSizeValue {
	fn default() -> Self {
		Self::Absolute(AbsoluteSize::Medium)
	}
}

// https://drafts.csswg.org/css-fonts-4/#propdef-font-size
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontFamilyValue {
	Named(Atom),
	// https://drafts.csswg.org/css-fonts-4/#generic-font-families
	#[default]
	Serif,
	SansSerif,
	Cursive,
	Fantasy,
	Monospace,
	SystemUi,
	Emoji,
	Math,
	Fangsong,
	UiSerif,
	UiSansSerif,
	UiMonospace,
	UiRounded,
}

// https://drafts.csswg.org/css-fonts/#propdef-font-style
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontStyleValue<'a> {
	#[default]
	Normal,
	Italic,
	Oblique(Spanned<MathExpr<'a, Angle>>),
}
