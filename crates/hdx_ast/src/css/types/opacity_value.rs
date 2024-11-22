use hdx_atom::atom;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::CSSFloat;

// CSS floats are different to f32s in that they do not represent NaN
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum OpacityValue {
	Number(CSSFloat),
	Percentage(CSSFloat),
}

impl OpacityValue {
	#[allow(non_upper_case_globals)]
	pub const Zero: OpacityValue = OpacityValue::Number(CSSFloat::Zero);

	pub fn to_f32(&self) -> f32 {
		match self {
			Self::Number(f) => (*f).into(),
			Self::Percentage(f) => (*f).into(),
		}
	}
}
impl From<i32> for OpacityValue {
	fn from(value: i32) -> Self {
		Self::Number((value as f32).into())
	}
}

impl From<f32> for OpacityValue {
	fn from(value: f32) -> Self {
		Self::Number(value.into())
	}
}

impl From<OpacityValue> for i32 {
	fn from(value: OpacityValue) -> Self {
		value.into()
	}
}

impl From<OpacityValue> for f32 {
	fn from(value: OpacityValue) -> Self {
		value.to_f32()
	}
}

impl<'a> Peek<'a> for OpacityValue {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Number]>().or_else(|| p.peek::<T![Dimension]>())
	}
}

impl<'a> Parse<'a> for OpacityValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = p.peek::<T![Number]>() {
			p.hop(token);
			return Ok(Self::Number(p.parse_number(token).into()));
		}
		let token = *p.parse::<T![Dimension]>()?;
		let atom = p.parse_atom_lower(token);
		if atom != atom!("%") {
			Err(diagnostics::UnexpectedDimension(atom, token.span()))?
		}
		Ok(Self::Percentage(p.parse_number(token).into()))
	}
}

impl<'a> WriteCss<'a> for OpacityValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Number(f) => write_css!(sink, f),
			Self::Percentage(f) => write_css!(sink, f, '%'),
		}
		Ok(())
	}
}
