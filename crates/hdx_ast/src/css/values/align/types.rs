use hdx_derive::{Atomizable, Parsable};
use hdx_derive::{Peekable, Writable};
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

pub(crate) use crate::css::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::{
	ColumnGap, /*AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf, */ RowGap,
};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(First, atom!("first"));
	custom_keyword!(Last, atom!("last"));
	custom_keyword!(Baseline, atom!("baseline"));
}

// https://drafts.csswg.org/css-align-3/#typedef-baseline-position
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BaselinePosition {
	First,
	Last,
	Baseline,
}

impl<'a> Parse<'a> for BaselinePosition {
	fn parse(p: &mut Parser) -> ParserResult<Self> {
		if let Some(token) = p.peek::<kw::Baseline>() {
			p.hop(token);
			Ok(Self::Baseline)
		} else if let Some(token) = p.peek::<kw::First>() {
			p.hop(token);
			p.parse::<kw::Baseline>()?;
			Ok(Self::First)
		} else {
			p.parse::<kw::First>()?;
			p.parse::<kw::Baseline>()?;
			Ok(Self::Last)
		}
	}
}

impl<'a> WriteCss<'a> for BaselinePosition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Baseline => write_css!(sink, kw::Baseline::atom()),
			Self::First => write_css!(sink, kw::First::atom(), ' ', kw::Baseline::atom()),
			Self::Last => write_css!(sink, kw::Last::atom(), ' ', kw::Baseline::atom()),
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-align-3/#typedef-overflow-position
#[derive(Atomizable, Writable, Parsable, Peekable, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum OverflowPosition {
	Unsafe, // atom!("unsafe")
	Safe,   // atom!("safe")
}

// https://drafts.csswg.org/css-align-3/#typedef-self-position
#[derive(Atomizable, Writable, Parsable, Peekable, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SelfPosition {
	Center,    // atom!("center")
	Start,     // atom!("start")
	End,       // atom!("end")
	SelfStart, // atom!("self-start")
	SelfEnd,   // atom!("self-end")
	FlexStart, // atom!("flex-start")
	FlexEnd,   // atom!("flex-end")
}
