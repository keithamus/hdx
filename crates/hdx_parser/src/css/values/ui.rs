use hdx_ast::css::values::CursorValue;

use crate::{diagnostics, Atomizable, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for CursorValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => {
				if let Some(val) = CursorValue::from_atom(*ident) {
					Ok(val.spanned(parser.advance()))
				} else {
					Err(diagnostics::UnexpectedIdent(*ident, parser.span()))?
				}
			}
			token => Err(diagnostics::Unexpected(*token, parser.span()))?,
		}
	}
}
