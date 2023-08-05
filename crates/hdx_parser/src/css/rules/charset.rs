use hdx_ast::css::rules::charset::Charset;
use oxc_allocator::Vec;

use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for Charset {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_at_keyword_of(atom!("charset"))?;
		let encoding = parser.expect_string()?;
		parser.expect(Kind::Semicolon)?;
		Ok(Self { encoding }.spanned(span.up_to(&parser.cur().span)))
	}
}
