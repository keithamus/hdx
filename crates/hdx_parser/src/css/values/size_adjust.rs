use hdx_ast::css::values::TextSizeAdjustValue;

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for TextSizeAdjustValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				match ident {
					atom!("none") => {
						Ok(TextSizeAdjustValue::None.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("auto") => {
						Ok(TextSizeAdjustValue::Auto.spanned(span.up_to(&parser.cur().span)))
					}
					_ => Err(diagnostics::UnexpectedIdent(ident, span))?,
				}
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(TextSizeAdjustValue::Percentage(value).spanned(span.up_to(&parser.cur().span)))
			}
			k => Err(diagnostics::Unexpected(k, span))?,
		}
	}
}
