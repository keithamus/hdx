use crate::macros::keyword_typedef;
use hdx_lexer::Span;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

pub use crate::css::units::*;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Infinite, atom!("infinite"));
}

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Debug, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SingleAnimationIterationCount {
	Infinite,
	Number(CSSFloat),
}

impl From<f32> for SingleAnimationIterationCount {
	fn from(f: f32) -> Self {
		Self::Number(f.into())
	}
}

impl<'a> Peek<'a> for SingleAnimationIterationCount {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<kw::Infinite>().or_else(|| p.peek::<CSSFloat>())
	}
}

impl<'a> Parse<'a> for SingleAnimationIterationCount {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = p.peek::<kw::Infinite>() {
			p.hop(token);
			return Ok(Self::Infinite);
		}
		let start = p.offset();
		let f = p.parse::<CSSFloat>()?;
		if f < 0.0 {
			Err(diagnostics::NumberTooSmall(f.into(), Span::new(start, p.offset())))?
		}
		Ok(Self::Number(f))
	}
}

impl<'a> WriteCss<'a> for SingleAnimationIterationCount {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Infinite => kw::Infinite::atom().write_css(sink),
			Self::Number(f) => f.write_css(sink),
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
