use hdx_ast::css::{
	unknown::{UnknownAtRule, UnknownDeclaration, UnknownPrelude, UnknownRule},
	values::ValueLike,
};
use hdx_lexer::Kind;
use miette::Result;

use crate::{atom, Atom, Parse, Parser, Spanned};

impl<'a> Parse<'a> for UnknownAtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			None,
			|parser: &mut Parser<'a>, name: Atom, prelude, rules, properties| {
				Ok(Self {
					name,
					prelude: parser.boxup(prelude),
					rules: parser.boxup(rules),
					properties: parser.boxup(properties),
				}
				.spanned(span.up_to(&parser.cur().span)))
			},
		)
	}
}

impl<'a> Parse<'a> for UnknownRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_qualified_rule(
			Some(Kind::Semicolon),
			true,
			|parser: &mut Parser<'a>, prelude, rules, properties| {
				Ok(Self {
					prelude: parser.boxup(prelude),
					rules: parser.boxup(rules),
					properties: parser.boxup(properties),
				}
				.spanned(span.up_to(&parser.cur().span)))
			},
		)
	}
}

impl<'a> Parse<'a> for UnknownPrelude<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let value = parser.parse_component_values(Kind::Semicolon, false)?;
		Ok(Self { value: parser.boxup(value) }.spanned(span.up_to(&parser.cur().span)))
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
impl<'a> Parse<'a> for UnknownDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut name = atom!("");
		let mut value_like = Spanned::dummy(ValueLike::Unknown);
		let value;
		// While consume-the-remnants-of-a-bad-declaration just returns token soup, we can at least
		// try to discover a useful property name, so try to do that first but if that fails then
		// shove it all in component values
		if parser.at(Kind::Ident) {
			let ident = parser.cur().as_atom().unwrap();
			if parser.peek().kind == Kind::Colon {
				parser.advance(); // ident
				parser.advance(); // colon
				name = ident;
				let checkpoint = parser.checkpoint();
				value_like = ValueLike::parse(parser).unwrap_or(value_like);
				parser.rewind(checkpoint);
				value = parser.parse_component_values(Kind::Semicolon, true)?;
			} else {
				value = parser.parse_component_values(Kind::Semicolon, true)?;
			}
		} else {
			value = parser.parse_component_values(Kind::Semicolon, true)?;
		}
		if parser.cur().kind == Kind::Semicolon {
			parser.advance();
		}
		Ok(Self { name, value_like, value: parser.boxup(value), important: false }
			.spanned(span.up_to(&parser.cur().span)))
	}
}

#[cfg(test)]
mod test {

	use hdx_ast::css::{
		component_values::ComponentValue,
		unknown::UnknownDeclaration,
		values::{ColorValue, Expr, NamedColor},
	};
	use hdx_lexer::{Token, TokenValue};

	use super::*;
	use crate::{Allocator, Atom, ParserOptions, Span, Spanned};

	#[test]
	fn parses_unknown_property() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "colour    :/**/ red;", ParserOptions::default());
		let mut component_values = parser.new_vec();
		component_values.push(Spanned {
			span: Span::new(16, 19),
			node: ComponentValue::Token(Token {
				kind: Kind::Ident,
				span: Span::new(16, 19),
				escaped: false,
				value: TokenValue::String(Atom::from("red")),
			}),
		});
		let expected = Spanned {
			span: Span::new(0, 20),
			node: UnknownDeclaration {
				name: Atom::from("colour"),
				value: parser.boxup(component_values),
				value_like: Spanned {
					span: Span::new(16, 19),
					node: ValueLike::Color(parser.boxup(Spanned {
						span: Span::new(16, 19),
						node: Expr::Literal(Spanned {
							span: Span::new(16, 19),
							node: ColorValue::Named(NamedColor::Red),
						}),
					})),
				},
				important: false,
			},
		};
		let parser_return = parser.parse_entirely_with::<UnknownDeclaration>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, expected);
	}
}
