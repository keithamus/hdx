use hdx_ast::css::{
	properties::Property,
	stylesheet::{CSSStyleRule, SelectorSet},
};

use crate::{diagnostics, Parse, Parser, Result, Spanned, Vec};

impl<'a> Parse<'a> for CSSStyleRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_qualified_rule(
			None,
			false,
			|parser: &mut Parser<'a>,
			 selectors: Option<Spanned<SelectorSet<'a>>>,
			 rules: Vec<'a, Spanned<CSSStyleRule<'a>>>,
			 declarations: Vec<'a, Spanned<Property<'a>>>| {
				if selectors.is_none() {
					Err(diagnostics::NoSelector(span, span.up_to(&parser.cur().span)))?
				}
				Ok(Self {
					selectors: parser.boxup(selectors.unwrap()),
					declarations: parser.boxup(declarations),
					rules: parser.boxup(rules),
				}
				.spanned(span.up_to(&parser.cur().span)))
			},
		)
	}
}
