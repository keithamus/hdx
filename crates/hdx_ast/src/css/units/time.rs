use hdx_atom::{atom, Atom};
use hdx_parser::{Dimension, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use super::{AbsoluteUnit, CSSFloat};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Time {
	Zero,
	Ms(CSSFloat),
	S(CSSFloat),
}

impl Time {
	pub fn new(val: CSSFloat, unit: Atom) -> Option<Self> {
		match unit {
			atom!("ms") => Some(Self::Ms(val.into())),
			atom!("s") => Some(Self::S(val.into())),
			_ => None,
		}
	}
}

impl Into<f32> for Time {
	fn into(self) -> f32 {
		match self {
			Self::Zero => 0.0,
			Self::Ms(f) => f.into(),
			Self::S(f) => f.into(),
		}
	}
}

impl Into<CSSFloat> for Time {
	fn into(self) -> CSSFloat {
		match self {
			Self::Zero => 0.0.into(),
			Self::Ms(f) | Self::S(f) => f,
		}
	}
}

impl AbsoluteUnit for Time {
	fn to_base(&self) -> Self {
		Self::S(match self {
			Self::Zero => 0.0.into(),
			Self::Ms(f) => *f / 1000.0,
			Self::S(f) => *f,
		})
	}
}

impl<'a> Peek<'a> for Time {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser
			.peek::<Token![Number]>()
			.filter(|token| token.stored_small_number() == Some(0.0))
			.or_else(|| parser.peek::<Dimension![Ms]>())
			.or_else(|| parser.peek::<Dimension![S]>())
	}
}

impl<'a> Parse<'a> for Time {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<Dimension![Ms]>() {
			parser.hop(token);
			Ok(Self::Ms(parser.parse_number(token).into()))
		} else {
			let token = *parser.parse::<Dimension![S]>()?;
			Ok(Self::Ms(parser.parse_number(token).into()))
		}
	}
}

impl<'a> WriteCss<'a> for Time {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Zero => write_css!(sink, '0'),
			Self::Ms(f) => write_css!(sink, f, <Dimension![Ms]>::atom()),
			Self::S(f) => write_css!(sink, f, <Dimension![S]>::atom()),
		};
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Time, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Time, "0s");
		// Truncates to 7dp
		assert_parse!(Time, "1.2345678901234s", "1.2345679s");
		// Removes redundant dp
		assert_parse!(Time, "-1.0s", "-1s");
	}
}
