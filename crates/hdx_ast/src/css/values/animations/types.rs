use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, keyword_typedef, Parse, Parser, Peek, Result as ParserResult};

pub use crate::css::units::*;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Infinite, atom!("infinite"));
}

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SingleAnimationIterationCount {
	Infinite(kw::Infinite),
	Number(CSSFloat),
}

impl<'a> Peek<'a> for SingleAnimationIterationCount {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<kw::Infinite>() || p.peek::<CSSFloat>()
	}
}

impl<'a> Parse<'a> for SingleAnimationIterationCount {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<kw::Infinite>() {
			return Ok(Self::Infinite(p.parse::<kw::Infinite>()?));
		}
		let int = p.parse::<CSSFloat>()?;
		let f: f32 = int.into();
		if f < 0.0 {
			let c: Cursor = int.into();
			Err(diagnostics::NumberTooSmall(f, c.into()))?
		}
		Ok(Self::Number(int))
	}
}

impl From<SingleAnimationIterationCount> for Cursor {
	fn from(value: SingleAnimationIterationCount) -> Self {
		match value {
			SingleAnimationIterationCount::Infinite(c) => c.into(),
			SingleAnimationIterationCount::Number(c) => c.into(),
		}
	}
}

// https://drafts.csswg.org/css-animations/#typedef-single-animation-direction
// <single-animation-direction> = normal | reverse | alternate | alternate-reverse
keyword_typedef!(SingleAnimationDirection {
	Normal: atom!("normal"),
	Reverse: atom!("reverse"),
	Alternate: atom!("alternate"),
	AlternateReverse: atom!("alternate-reverse"),
});

// https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state
// <single-animation-play-state> = running | paused
keyword_typedef!(SingleAnimationPlayState { Running: atom!("running"), Paused: atom!("paused") });

// https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode
// <single-animation-fill-mode> = none | forwards | backwards | both
keyword_typedef!(SingleAnimationFillMode {
	None: atom!("none"),
	Forwards: atom!("forwards"),
	Backwards: atom!("backwards"),
	Both: atom!("both"),
});

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition
// <single-animation-composition> = replace | add | accumulate
keyword_typedef!(SingleAnimationComposition {
	Replace: atom!("replace"),
	Add: atom!("add"),
	Accumulate: atom!("accumulate"),
});
