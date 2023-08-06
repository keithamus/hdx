use hdx_ast::css::values::Angle;

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for Angle {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("deg"),
						parser.cur().span,
					))?
				}
				parser.advance();
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			Kind::Dimension => {
				let value = parser.cur().value.as_f32().unwrap();
				let unit = parser.cur_atom().unwrap();
				parser.advance();
				match unit {
					atom!("deg") => Ok(Self::Deg(value).spanned(span.until(parser.cur().span))),
					atom!("grad") => Ok(Self::Grad(value).spanned(span.until(parser.cur().span))),
					atom!("rad") => Ok(Self::Rad(value).spanned(span.until(parser.cur().span))),
					atom!("turn") => Ok(Self::Turn(value).spanned(span.until(parser.cur().span))),
					_ => Err(diagnostics::UnexpectedIdent(unit, parser.cur().span))?,
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
	}
}
