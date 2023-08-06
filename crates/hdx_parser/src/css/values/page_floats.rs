use hdx_ast::css::values::{
	page_floats::{FloatDeferValue, FloatValue, SnapBlockFloat, SnapInlineFloat},
	Length,
};

use crate::{atom, diagnostics, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for FloatDeferValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = Self::from_atom(parser.expect_ident()?) {
					Ok(val.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					))?
				}
			}
			Kind::Number => {
				let node = parser.expect_int()?;
				Ok(Self::Integer(node).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

impl<'a> Parse<'a> for FloatValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = Self::from_atom(parser.expect_ident()?) {
					Ok(val.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					))?
				}
			}
			Kind::Function => {
				let name = parser.expect_function()?;
				let length = Length::parse(parser)?;
				parser.expect(Kind::Comma)?;
				let floated_atom = parser.expect_ident()?;
				match name {
					atom!("snap-block") => {
						if let Some(floated) = SnapBlockFloat::from_atom(floated_atom) {
							Ok(Self::SnapBlockFunction(length, floated)
								.spanned(span.until(parser.cur().span)))
						} else {
							Err(diagnostics::UnexpectedIdent(
								parser.cur_atom().unwrap(),
								parser.cur().span,
							)
							.into())
						}
					}
					atom!("snap-inline") => {
						if let Some(floated) = SnapInlineFloat::from_atom(floated_atom) {
							Ok(Self::SnapInlineFunction(length, floated)
								.spanned(span.until(parser.cur().span)))
						} else {
							Err(diagnostics::UnexpectedIdent(
								parser.cur_atom().unwrap(),
								parser.cur().span,
							)
							.into())
						}
					}
					_ => Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					)
					.into()),
				}
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}
