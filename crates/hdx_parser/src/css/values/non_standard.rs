use hdx_ast::css::values::non_standard::ZoomValue;

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for ZoomValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				match ident.to_ascii_lowercase() {
					atom!("normal") => Ok(Self::Normal.spanned(span.until(parser.cur().span))),
					atom!("reset") => Ok(Self::Reset.spanned(span.until(parser.cur().span))),
					_ => Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					))?,
				}
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Percentage(value).spanned(span.until(parser.cur().span)))
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Number(value).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}
