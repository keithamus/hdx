use hdx_lexer::Token;
use hdx_parser::{
	diagnostics::{NumberNotNegative, UnexpectedDimension},
	unexpected, Parse,
};

use crate::css::values::units::LengthPercentage;

// https://drafts.csswg.org/css-box-4/#padding-physical
pub type PaddingRight = LengthPercentage;

impl<'a> Parse<'a> for PaddingRight {
	fn parse(parser: &mut hdx_parser::Parser<'a>) -> miette::Result<hdx_parser::Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Number(n, _) if n == 0.0 => Ok(Self::Zero.spanned(span)),
			Token::Dimension(n, unit, _) if n == 0.0 => {
				if n < 0 {
					Err(NumberNotNegative(n, span))?
				}
				if let Some(val) = Self::new(n.into(), unit) {
					Ok(val.spanned(span))
				} else {
					Err(UnexpectedDimension(unit, span))?
				}
			}
			token => unexpected!(parser, token),
		}
	}
}
