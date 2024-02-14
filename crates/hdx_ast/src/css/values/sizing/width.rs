use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, expect, unexpected, Parse, Parser, Result as ParserResult, Spanned};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::LengthPercentage;
use crate::Writable;

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Width {
	#[default]
	Auto, // atom!("auto")
	MinContent, // atom!("min-content")
	MaxContent, // atom!("max-content")  TODO: `intrinsic` non standard
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,    // atom!("stretch")  TODO: -webkit-fill-available, -moz-available
	FitContent, // atom!("fit-content")
	Contain,    // atom!("contain")

	LengthPercentage(LengthPercentage),
	#[writable(as_function = "fit-content")]
	FitContentFunction(LengthPercentage),
}

impl<'a> Parse<'a> for Width {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Ident(atom!("auto")) => {
				parser.advance();
				Ok(Self::Auto.spanned(span))
			}
			Token::Ident(atom!("min-content")) => {
				parser.advance();
				Ok(Self::MinContent.spanned(span))
			}
			Token::Ident(atom!("max-content")) => {
				parser.advance();
				Ok(Self::MaxContent.spanned(span))
			}
			Token::Ident(atom!("stretch")) => {
				parser.advance();
				Ok(Self::Stretch.spanned(span))
			}
			Token::Ident(atom!("fit-content")) => {
				parser.advance();
				Ok(Self::FitContent.spanned(span))
			}
			Token::Ident(atom!("contain")) => {
				parser.advance();
				Ok(Self::Contain.spanned(span))
			}
			Token::Dimension(val, unit, _) => {
				if val < 0.0 {
					Err(diagnostics::NumberNotNegative(val, span))?
				}
				if let Some(val) = LengthPercentage::new(val.into(), unit.clone()) {
					parser.advance();
					Ok(Self::LengthPercentage(val).spanned(span))
				} else {
					Err(diagnostics::UnexpectedDimension(unit, span))?
				}
			}
			Token::Number(val, _) if val == 0.0 => {
				parser.advance();
				Ok(Self::LengthPercentage(LengthPercentage::Zero).spanned(span))
			}
			Token::Function(atom!("fit-content")) => {
				parser.advance();
				match parser.cur() {
					Token::Dimension(val, unit, _) => {
						if val < 0.0 {
							Err(diagnostics::NumberNotNegative(val, span))?
						}
						if let Some(val) = LengthPercentage::new(val.into(), unit.clone()) {
							parser.advance();
							expect!(parser, Token::RightParen);
							parser.advance();
							Ok(Self::FitContentFunction(val).spanned(span))
						} else {
							Err(diagnostics::UnexpectedDimension(unit, span))?
						}
					}
					Token::Number(val, _) if val == 0.0 => {
						parser.advance();
						expect!(parser, Token::RightParen);
						parser.advance();
						Ok(Self::FitContentFunction(LengthPercentage::Zero).spanned(span))
					}
					token => unexpected!(parser, token),
				}
			}
			token => unexpected!(parser, token),
		}
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Width>(), 12);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Width>(&allocator, "0", "0");
		test_write::<Width>(&allocator, "1px", "1px");
		test_write::<Width>(&allocator, "fit-content", "fit-content");
		test_write::<Width>(&allocator, "fit-content(20rem)", "fit-content(20rem)");
		test_write::<Width>(&allocator, "fit-content(0)", "fit-content(0)");
	}
}
