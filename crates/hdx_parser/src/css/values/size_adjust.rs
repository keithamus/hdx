use hdx_ast::css::values::TextSizeAdjustValue;

use crate::{atom, diagnostics, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for TextSizeAdjustValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("none") => Ok(TextSizeAdjustValue::None.spanned(parser.advance())),
				atom!("auto") => Ok(TextSizeAdjustValue::Auto.spanned(parser.advance())),
				_ => Err(diagnostics::UnexpectedIdent(*ident, parser.span()))?,
			},
			Token::Dimension(_, value, atom!("%")) => {
				Ok(TextSizeAdjustValue::Percentage(*value).spanned(parser.advance()))
			}
			token => Err(diagnostics::Unexpected(*token, parser.span()))?,
		}
	}
}
