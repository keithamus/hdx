use hdx_ast::css::{
	properties::Property,
	rules::page::{
		PageMarginBox, PageMarginRule, PagePseudoClass, PageRule, PageSelector, PageSelectorList,
	},
};
use oxc_allocator::Vec;

use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for PageRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			Some(atom!("page")),
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 selectors: Option<Spanned<PageSelectorList<'a>>>,
			 rules: Vec<'a, Spanned<PageMarginRule<'a>>>,
			 properties: Vec<'a, Spanned<Property<'a>>>| {
				Ok(Self {
					selectors: parser.boxup(selectors),
					properties: parser.boxup(properties),
					rules: parser.boxup(rules),
				}
				.spanned(span.up_to(&parser.cur().span)))
			},
		)
	}
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(Self { children: parser.parse_comma_list_of::<PageSelector>()? }
			.spanned(span.up_to(&parser.cur().span)))
	}
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut page_type = None;
		let mut pseudos = parser.new_vec();
		if parser.at(Kind::Ident) {
			println!("PageSelector::page_type assigning to {:?}", parser.cur());
			page_type = Some(parser.expect_ident()?);
		} else {
			parser.expect_without_advance(Kind::Colon)?;
		}
		if parser.at(Kind::Colon) {
			loop {
				if !parser.at(Kind::Colon) {
					break;
				}
				pseudos.push(PagePseudoClass::parse(parser)?);
			}
		}
		println!("PageSelector::OK(self) {:?} {:?}", page_type, pseudos);
		Ok(Self { page_type, pseudos }.spanned(span.up_to(&parser.cur().span)))
	}
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect(Kind::Colon)?;
		let name = parser.expect_ident()?;
		match Self::from_atom(name.clone()) {
			Some(v) => Ok(v.spanned(span.up_to(&parser.cur().span))),
			_ => Err(diagnostics::UnexpectedPseudo(name, span).into()),
		}
	}
}

impl<'a> Parse<'a> for PageMarginRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			None,
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 _prelude: Option<Spanned<IgnoreWhitespaceInPageMarginPrelude>>,
			 _rules: Vec<'a, Spanned<PageMarginRule<'a>>>,
			 properties: Vec<'a, Spanned<Property<'a>>>| {
				Ok(Self { margin_box: PageMarginBox::TopLeft, properties }
					.spanned(span.up_to(&parser.cur().span)))
			},
		)
	}
}

struct IgnoreWhitespaceInPageMarginPrelude;
impl<'a> Parse<'a> for IgnoreWhitespaceInPageMarginPrelude {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_without_advance(Kind::LeftCurly)?;
		Ok(Self {}.spanned(span.up_to(&parser.cur().span)))
	}
}
