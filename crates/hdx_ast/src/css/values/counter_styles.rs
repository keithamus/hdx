#[cfg(feature = "serde")]
use serde::Serialize;

use super::Image;
use crate::{atom, Atom, Atomizable, Box, Spanned, Vec};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CounterStyle<'a> {
	Named(Atom),
	Symbols(Spanned<Symbols<'a>>),
	Predefined(PredefinedCounterStyle),
}

// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum PredefinedCounterStyle {
	Decimal,            // atom!("decimal"),
	DecimalLeadingZero, // atom!("decimal-leading-zero"),
	ArabicIndic,        // atom!("arabic-indic"),
	Armenian,           // atom!("armenian"),
	UpperArmenian,      // atom!("upper-armenian"),
	LowerArmenian,      // atom!("lower-armenian"),
	Bengali,            // atom!("bengali"),
	Cambodian,          // atom!("cambodian"),
	Khmer,              // atom!("khmer"),
	CjkDecimal,         // atom!("cjk-decimal"),
	Devanagari,         // atom!("devanagari"),
	Georgian,           // atom!("georgian"),
	Gujarati,           // atom!("gujarati"),
	Gurmukhi,           // atom!("gurmukhi"),
	Hebrew,             // atom!("hebrew"),
	Kannada,            // atom!("kannada"),
	Lao,                // atom!("lao"),
	Malayalam,          // atom!("malayalam"),
	Mongolian,          // atom!("mongolian"),
	Myanmar,            // atom!("myanmar"),
	Oriya,              // atom!("oriya"),
	Persian,            // atom!("persian"),
	LowerRoman,         // atom!("lower-roman"),
	UpperRoman,         // atom!("upper-roman"),
	Tamil,              // atom!("tamil"),
	Telugu,             // atom!("telugu"),
	Thai,               // atom!("thai"),
	Tibetan,            // atom!("tibetan"),
	LowerAlpha,         // atom!("lower-alpha"),
	UpperAlpha,         // atom!("upper-alpha"),
	UpperLatin,         // atom!("upper-latin"),
	LowerGreek,         // atom!("lower-greek"),
	Hiragana,           // atom!("hiragana"),
	HiraganaIroha,      // atom!("hiragana-iroha"),
	Katakana,           // atom!("katakana"),
	KatakanaIroha,      // atom!("katakana-iroha"),
	#[default]
	Disc,     // atom!("disc"),
	Square,             // atom!("square"),
	DisclousureOpen,    // atom!("disclousure-open"),
	DisclousureClosed,  // atom!("disclousure-closed"),
	CjkEarthlyBranch,   // atom!("cjk-earthly-branch"),
	CjkHeavenlyStem,    // atom!("cjk-heavenly-stem"),
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Symbols<'a> {
	kind: SymbolsType,
	symbols: Box<'a, Vec<'a, Symbol<'a>>>,
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Symbol<'a> {
	String(Atom),
	Image(Box<'a, Image<'a>>),
}

// https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum SymbolsType {
	Cyclic,     // atom!("cyclic")
	Numeric,    // atom!("numeric")
	Alphabetic, // atom!("alphabetic")
	#[default]
	Symbolic, // atom!("symbolic")
	Fixed,      // atom!("fixed")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<CounterStyle>(), 24);
		assert_eq!(size_of::<Symbols>(), 16);
		assert_eq!(size_of::<Symbol>(), 16);
		assert_eq!(size_of::<SymbolsType>(), 1);
	}
}
