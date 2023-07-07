use hdx_ast::css::values::CursorValue;

use crate::{diagnostics, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for CursorValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let span = parser.cur().span;
				let ident = parser.expect_ident()?;
				if let Some(val) = CursorValue::from_atom(ident.clone()) {
					Ok(val.spanned(span.up_to(&parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, span))?
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
	}
}
