use hdx_ast::css::{
	rules::page::PageRule,
	selector::Selector,
	stylesheet::{AtRule, AtRuleId, SelectorSet, StyleRule, Stylesheet, StylesheetRule},
	unknown::{UnknownAtRule, UnknownRule},
};
use hdx_lexer::Kind;

use crate::{Parse, Parser, Result, Span, Spanned};

// https://drafts.csswg.org/css-syntax-3/#consume-stylesheet-contents
impl<'a> Parse<'a> for Stylesheet<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut rules = parser.new_vec();
		loop {
			match parser.cur().kind {
				Kind::Eof => break,
				Kind::Comment | Kind::Whitespace | Kind::Cdc | Kind::Cdo => parser.advance(),
				Kind::AtKeyword => {
					let rule = AtRule::parse(parser)?;
					rules.push(StylesheetRule::At(parser.boxup(rule)));
				}
				_ => {
					// The spec talks of QualifiedRules but in the context of a Stylesheet
					// the only non-At Rule is a StyleRule, so parse that:
					let checkpoint = parser.checkpoint();
					match StyleRule::parse(parser) {
						Ok(rule) => rules.push(StylesheetRule::Style(parser.boxup(rule))),
						Err(err) => {
							parser.rewind(checkpoint);
							parser.warnings.push(err);
							let rule = UnknownRule::parse(parser)?;
							rules.push(StylesheetRule::Unknown(parser.boxup(rule)));
						}
					}
				}
			}
		}
		Ok(Stylesheet { rules }.spanned(span.up_to(&parser.cur().span)))
	}
}

impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_without_advance(Kind::AtKeyword)?;
		Ok(match AtRuleId::from_atom(parser.cur().as_atom_lower().unwrap()) {
			AtRuleId::Page => {
				let rule = PageRule::parse(parser)?;
				AtRule::Page(parser.boxup(rule)).spanned(span.up_to(&parser.cur().span))
			}
			_ => {
				let rule = UnknownAtRule::parse(parser)?;
				AtRule::Unknown(parser.boxup(rule)).spanned(span.up_to(&parser.cur().span))
			}
		})
	}
}

impl<'a> Parse<'a> for SelectorSet<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(Self { children: parser.parse_comma_list_of::<Selector>()? }
			.spanned(span.up_to(&parser.cur().span)))
	}
}

#[cfg(test)]
mod test {
	use oxc_allocator::Allocator;

	use super::Stylesheet;
	use crate::{Parser, ParserOptions};

	#[test]
	fn smoke_test() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "", ParserOptions::default());
		let parser_return = parser.parse_with::<Stylesheet>();
		let ast = parser_return.output.unwrap();
		assert_eq!(ast.node.rules.len(), 0);
	}

	#[test]
	fn parses_two_rules() {
		let allocator = Allocator::default();
		let parser = Parser::new(
			&allocator,
			"a{overflow:hidden !important;position:relative}.b{}",
			ParserOptions::default(),
		);
		let parser_return = parser.parse_with::<Stylesheet>();
		let ast = parser_return.output.unwrap();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		assert_eq!(ast.node.rules.len(), 2);
	}
}
