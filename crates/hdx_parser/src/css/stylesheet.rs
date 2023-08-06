use hdx_ast::css::{
	rules::{CSSCharsetRule, CSSPageRule},
	selector::Selector,
	stylesheet::{AtRuleId, CSSRule, CSSStyleRule, CSSStyleSheet, SelectorSet},
	unknown::{UnknownAtRule, UnknownRule},
};
use hdx_lexer::Kind;

use crate::{diagnostics, Atomizable, Parse, Parser, Result, Span, Spanned};

// https://drafts.csswg.org/css-syntax-3/#consume-stylesheet-contents
impl<'a> Parse<'a> for CSSStyleSheet<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut rules = parser.new_vec();
		loop {
			match parser.cur().kind {
				Kind::Eof => break,
				Kind::Comment | Kind::Whitespace | Kind::Cdc | Kind::Cdo => parser.advance(),
				Kind::AtKeyword => {
					rules.push(match AtRuleId::from_atom(parser.cur_atom_lower().unwrap()) {
						Some(AtRuleId::Charset) => {
							let rule = CSSCharsetRule::parse(parser)?;
							CSSRule::Charset(parser.boxup(rule))
						}
						Some(AtRuleId::Page) => {
							let rule = CSSPageRule::parse(parser)?;
							CSSRule::Page(parser.boxup(rule))
						}
						None => {
							let rule = UnknownAtRule::parse(parser)?;
							parser.warnings.push(diagnostics::UnknownRule(rule.span).into());
							CSSRule::UnknownAt(parser.boxup(rule))
						}
					});
				}
				_ => {
					// The spec talks of QualifiedRules but in the context of a Stylesheet
					// the only non-At Rule is a StyleRule, so parse that:
					let checkpoint = parser.checkpoint();
					match CSSStyleRule::parse(parser) {
						Ok(rule) => rules.push(CSSRule::Style(parser.boxup(rule))),
						Err(err) => {
							parser.rewind(checkpoint);
							parser.warnings.push(err);
							let rule = UnknownRule::parse(parser)?;
							rules.push(CSSRule::Unknown(parser.boxup(rule)));
						}
					}
				}
			}
		}
		Ok(Self { rules }.spanned(span.up_to(&parser.cur().span)))
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
	use hdx_ast::css::rules::CSSCharsetRule;
	use oxc_allocator::Allocator;

	use super::{CSSRule, CSSStyleSheet};
	use crate::{atom, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn smoke_test() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "", ParserOptions::default());
		let parser_return = parser.parse_with::<CSSStyleSheet>();
		let ast = parser_return.output.unwrap();
		assert_eq!(ast.node.rules.len(), 0);
	}

	#[test]
	fn parses_charset() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "@charset \"utf-8\";", ParserOptions::default());
		let mut rules = parser.new_vec();
		rules.push(CSSRule::Charset(parser.boxup(Spanned {
			span: Span::new(0, 17),
			node: CSSCharsetRule { encoding: atom!("utf-8") },
		})));
		let expected = Spanned { span: Span::new(0, 17), node: CSSStyleSheet { rules } };
		let parser_return = parser.parse_with::<CSSStyleSheet>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, expected);
	}

	#[test]
	fn parses_two_rules() {
		let allocator = Allocator::default();
		let parser = Parser::new(
			&allocator,
			"a{overflow:hidden !important;position:relative}.b{}",
			ParserOptions::default(),
		);
		let parser_return = parser.parse_with::<CSSStyleSheet>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast.node.rules.len(), 2);
	}
}
