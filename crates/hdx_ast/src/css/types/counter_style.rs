use hdx_parser::{keyword_typedef, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use super::Symbols;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum CounterStyle<'a> {
	Predefined(PredefinedCounter),
	Named(T![Ident]),
	Symbols(Symbols<'a>),
}

impl<'a> Peek<'a> for CounterStyle<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<T![Ident]>() || p.peek::<Symbols>()
	}
}

impl<'a> Parse<'a> for CounterStyle<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<PredefinedCounter>() {
			p.parse::<PredefinedCounter>().map(Self::Predefined)
		} else if p.peek::<T![Ident]>() {
			p.parse::<T![Ident]>().map(Self::Named)
		} else {
			p.parse::<Symbols>().map(Self::Symbols)
		}
	}
}

impl<'a> ToCursors for CounterStyle<'a> {
	fn to_cursors(&self, s: &mut impl hdx_parser::CursorSink) {
		match self {
			Self::Predefined(c) => s.append(c.into()),
			Self::Named(c) => s.append(c.into()),
			Self::Symbols(symbols) => ToCursors::to_cursors(symbols, s),
		}
	}
}

// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
keyword_typedef!(PredefinedCounter {
	Decimal: atom!("decimal"),
	DecimalLeadingZero: atom!("decimal-leading-zero"),
	ArabicIndic: atom!("arabic-indic"),
	Armenian: atom!("armenian"),
	UpperArmenian: atom!("upper-armenian"),
	LowerArmenian: atom!("lower-armenian"),
	Bengali: atom!("bengali"),
	Cambodian: atom!("cambodian"),
	Khmer: atom!("khmer"),
	CjkDecimal: atom!("cjk-decimal"),
	Devanagari: atom!("devanagari"),
	Georgian: atom!("georgian"),
	Gujarati: atom!("gujarati"),
	Gurmukhi: atom!("gurmukhi"),
	Hebrew: atom!("hebrew"),
	Kannada: atom!("kannada"),
	Lao: atom!("lao"),
	Malayalam: atom!("malayalam"),
	Mongolian: atom!("mongolian"),
	Myanmar: atom!("myanmar"),
	Oriya: atom!("oriya"),
	Persian: atom!("persian"),
	LowerRoman: atom!("lower-roman"),
	UpperRoman: atom!("upper-roman"),
	Tamil: atom!("tamil"),
	Telugu: atom!("telugu"),
	Thai: atom!("thai"),
	Tibetan: atom!("tibetan"),
	LowerAlpha: atom!("lower-alpha"),
	UpperAlpha: atom!("upper-alpha"),
	UpperLatin: atom!("upper-latin"),
	LowerGreek: atom!("lower-greek"),
	Hiragana: atom!("hiragana"),
	HiraganaIroha: atom!("hiragana-iroha"),
	Katakana: atom!("katakana"),
	KatakanaIroha: atom!("katakana-iroha"),
	Disc: atom!("disc"),
	Square: atom!("square"),
	DisclousureOpen: atom!("disclousure-open"),
	DisclousureClosed: atom!("disclousure-closed"),
	CjkEarthlyBranch: atom!("cjk-earthly-branch"),
	CjkHeavenlyStem: atom!("cjk-heavenly-stem"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CounterStyle, 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CounterStyle, "cjk-heavenly-stem");
		assert_parse!(CounterStyle, "foobar");
		assert_parse!(CounterStyle, "symbols(symbolic'+')");
	}
}
