use hdx_atom::{Atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_lexer::Token;
use hdx_parser::{Parse, Parser, Result as ParserResult, Spanned};

use super::Symbols;

#[derive(Writable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum CounterStyle {
	Named(Atom),
	Symbols(Spanned<Symbols>),
	Predefined(PredefinedCounterStyle),
}

impl Default for CounterStyle {
	fn default() -> Self {
		Self::Predefined(PredefinedCounterStyle::default())
	}
}

impl<'a> Parse<'a> for CounterStyle {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.peek().clone() {
			Token::Ident(atom) => {
				parser.advance();
				if let Some(style) = PredefinedCounterStyle::from_atom(&atom) {
					Ok(Self::Predefined(style))
				} else {
					Ok(Self::Named(atom))
				}
			}
			_ => Ok(Self::Symbols(Symbols::parse_spanned(parser)?)),
		}
	}
}

// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
#[derive(Writable, Atomizable, Default, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CounterStyle, 40);
		assert_size!(PredefinedCounterStyle, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CounterStyle, "cjk-heavenly-stem");
		assert_parse!(CounterStyle, "foobar");
		assert_parse!(CounterStyle, "symbols(symbolic '+')");
	}
}
