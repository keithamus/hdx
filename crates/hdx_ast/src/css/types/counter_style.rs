use hdx_atom::{atom, Atom};
use hdx_derive::Writable;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, Spanned, Token};

use super::Symbols;

#[derive(Writable, Default, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum CounterStyle<'a> {
	// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
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
	Disc,   // atom!("disc"),
	Square,             // atom!("square"),
	DisclousureOpen,    // atom!("disclousure-open"),
	DisclousureClosed,  // atom!("disclousure-closed"),
	CjkEarthlyBranch,   // atom!("cjk-earthly-branch"),
	CjkHeavenlyStem,    // atom!("cjk-heavenly-stem"),

	Named(Atom),
	Symbols(Spanned<Symbols<'a>>),
}

impl<'a> Peek<'a> for CounterStyle<'a> {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Token![Ident]>().or_else(|| parser.peek::<Symbols>())
	}
}

impl<'a> Parse<'a> for CounterStyle<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<Token![Ident]>() {
			let atom = parser.parse_atom(token);
			parser.hop(token);
			match atom {
				atom!("decimal") => Ok(Self::Decimal),
				atom!("decimal-leading-zero") => Ok(Self::DecimalLeadingZero),
				atom!("arabic-indic") => Ok(Self::ArabicIndic),
				atom!("armenian") => Ok(Self::Armenian),
				atom!("upper-armenian") => Ok(Self::UpperArmenian),
				atom!("lower-armenian") => Ok(Self::LowerArmenian),
				atom!("bengali") => Ok(Self::Bengali),
				atom!("cambodian") => Ok(Self::Cambodian),
				atom!("khmer") => Ok(Self::Khmer),
				atom!("cjk-decimal") => Ok(Self::CjkDecimal),
				atom!("devanagari") => Ok(Self::Devanagari),
				atom!("georgian") => Ok(Self::Georgian),
				atom!("gujarati") => Ok(Self::Gujarati),
				atom!("gurmukhi") => Ok(Self::Gurmukhi),
				atom!("hebrew") => Ok(Self::Hebrew),
				atom!("kannada") => Ok(Self::Kannada),
				atom!("lao") => Ok(Self::Lao),
				atom!("malayalam") => Ok(Self::Malayalam),
				atom!("mongolian") => Ok(Self::Mongolian),
				atom!("myanmar") => Ok(Self::Myanmar),
				atom!("oriya") => Ok(Self::Oriya),
				atom!("persian") => Ok(Self::Persian),
				atom!("lower-roman") => Ok(Self::LowerRoman),
				atom!("upper-roman") => Ok(Self::UpperRoman),
				atom!("tamil") => Ok(Self::Tamil),
				atom!("telugu") => Ok(Self::Telugu),
				atom!("thai") => Ok(Self::Thai),
				atom!("tibetan") => Ok(Self::Tibetan),
				atom!("lower-alpha") => Ok(Self::LowerAlpha),
				atom!("upper-alpha") => Ok(Self::UpperAlpha),
				atom!("upper-latin") => Ok(Self::UpperLatin),
				atom!("lower-greek") => Ok(Self::LowerGreek),
				atom!("hiragana") => Ok(Self::Hiragana),
				atom!("hiragana-iroha") => Ok(Self::HiraganaIroha),
				atom!("katakana") => Ok(Self::Katakana),
				atom!("katakana-iroha") => Ok(Self::KatakanaIroha),
				atom!("disc") => Ok(Self::Disc),
				atom!("square") => Ok(Self::Square),
				atom!("disclousure-open") => Ok(Self::DisclousureOpen),
				atom!("disclousure-closed") => Ok(Self::DisclousureClosed),
				atom!("cjk-earthly-branch") => Ok(Self::CjkEarthlyBranch),
				atom!("cjk-heavenly-stem") => Ok(Self::CjkHeavenlyStem),
				atom => Ok(Self::Named(atom)),
			}
		} else {
			Ok(Self::Symbols(Symbols::parse_spanned(parser)?))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CounterStyle, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CounterStyle, "cjk-heavenly-stem");
		assert_parse!(CounterStyle, "foobar");
		assert_parse!(CounterStyle, "symbols(symbolic '+')");
	}
}
