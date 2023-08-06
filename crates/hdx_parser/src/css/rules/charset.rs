use hdx_ast::css::rules::charset::CSSCharsetRule;

use crate::{atom, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for CSSCharsetRule {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_at_keyword_of(atom!("charset"))?;
		let encoding = parser.expect_string()?;
		parser.expect(Kind::Semicolon)?;
		Ok(Self { encoding }.spanned(span.up_to(&parser.cur().span)))
	}
}
