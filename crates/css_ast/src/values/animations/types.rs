use css_lexer::Cursor;
use css_parse::{diagnostics, keyword_set, Parse, Parser, Peek, Result as ParserResult, T};

pub use crate::units::*;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SingleAnimationIterationCount {
	Infinite(T![Ident]),
	Number(CSSFloat),
}

impl<'a> Peek<'a> for SingleAnimationIterationCount {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<CSSFloat>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "infinite"))
	}
}

impl<'a> Parse<'a> for SingleAnimationIterationCount {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() && p.eq_ignore_ascii_case(p.peek_n(1), "infinite") {
			return Ok(Self::Infinite(p.parse::<T![Ident]>()?));
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
keyword_set!(SingleAnimationDirection {
	Normal: "normal",
	Reverse: "reverse",
	Alternate: "alternate",
	AlternateReverse: "alternate-reverse",
});

// https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state
// <single-animation-play-state> = running | paused
keyword_set!(SingleAnimationPlayState { Running: "running", Paused: "paused" });

// https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode
// <single-animation-fill-mode> = none | forwards | backwards | both
keyword_set!(SingleAnimationFillMode { None: "none", Forwards: "forwards", Backwards: "backwards", Both: "both" });

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition
// <single-animation-composition> = replace | add | accumulate
keyword_set!(SingleAnimationComposition { Replace: "replace", Add: "add", Accumulate: "accumulate" });
