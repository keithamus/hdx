use hdx_ast::css::values::non_standard::ZoomValue;

use crate::{atom, diagnostics, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for ZoomValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("normal") => Ok(Self::Normal.spanned(parser.advance())),
				atom!("reset") => Ok(Self::Reset.spanned(parser.advance())),
				_ => Err(diagnostics::UnexpectedIdent(*ident, parser.span()))?,
			},
			Token::Dimension(_, value, atom!("%")) => {
				Ok(Self::Percentage(*value).spanned(parser.advance()))
			}
			Token::Number(_, value) => Ok(Self::Number(*value).spanned(parser.advance())),
			token => Err(diagnostics::Unexpected(*token, parser.span()))?,
		}
	}
}
