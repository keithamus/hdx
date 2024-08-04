use hdx_parser::{Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::{types::Color, units::LengthPercentageOrFlex};

mod func {
	use hdx_parser::custom_function;
	custom_function!(Stripes, atom!("stripes"));
}

// https://drafts.csswg.org/css-images-4/#typedef-image-1d
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Image1D(SmallVec<[ColorStripe; 1]>);

impl<'a> Peek<'a> for Image1D {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<func::Stripes>()
	}
}

impl<'a> Parse<'a> for Image1D {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let _token = parser.parse::<func::Stripes>()?;
		Ok(Self(smallvec![]))
	}
}

impl<'a> WriteCss<'a> for Image1D {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		todo!();
	}
}

// https://drafts.csswg.org/css-images-4/#typedef-color-stripe
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ColorStripe(pub Color, pub Option<LengthPercentageOrFlex>);
