use hdx_ast::css::values::{
	ColorValue, Expr, MathExpr, Shorthand, TextDecorationLineValue, TextDecorationShorthand,
	TextDecorationStyleValue,
};

use crate::{atom, diagnostics, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for TextDecorationShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut color = Shorthand::Implicit;
		let mut style = Shorthand::Implicit;
		let mut line = Shorthand::Implicit;
		loop {
			match parser.cur() {
				Token::Ident(ident) => {
					if style.is_implicit()
						&& matches!(
							ident.to_ascii_lowercase(),
							atom!("solid")
								| atom!("double") | atom!("dotted")
								| atom!("dashed") | atom!("wavy")
						) {
						let node = Expr::<TextDecorationStyleValue>::parse(parser)?;
						style = Shorthand::Explicit(parser.boxup(node));
					} else if line.is_implicit()
						&& matches!(
							*ident,
							atom!("none")
								| atom!("underline") | atom!("overline")
								| atom!("line-through") | atom!("blink")
						) {
						let node = Expr::<TextDecorationLineValue>::parse(parser)?;
						line = Shorthand::Explicit(parser.boxup(node));
					} else if color.is_implicit() {
						let node = MathExpr::<ColorValue>::parse(parser)?;
						color = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(ident.clone(), parser.span()))?
					}
				}
				Token::Semicolon | Token::Comma | Token::Eof => {
					break;
				}
				token => {
					if color.is_implicit() {
						let node = MathExpr::<ColorValue>::parse(parser)?;
						color = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::Unexpected(*token, parser.span()))?
					}
				}
			}
			if color.is_explicit() && style.is_explicit() && line.is_explicit() {
				break;
			}
		}
		Ok(Self { color, style, line }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for TextDecorationLineValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("none") => {
					return Ok(Self::None.spanned(parser.advance()));
				}
				_ => {}
			},
			_ => {}
		}
		let mut underline = false;
		let mut overline = false;
		let mut line_through = false;
		let mut blink = false;
		loop {
			match parser.cur() {
				Token::Ident(ident) => {
					match ident.to_ascii_lowercase() {
						atom!("underline") => {
							if underline {
								break;
							}
							underline = true
						}
						atom!("overline") => {
							if overline {
								break;
							}
							overline = true
						}
						atom!("line-through") => {
							if overline {
								break;
							}
							line_through = true
						}
						atom!("blink") => {
							if overline {
								break;
							}
							blink = true
						}
						_ => break,
					};
					parser.advance();
				}
				_ => break,
			}
		}
		Ok(Self::Style { underline, overline, line_through, blink }.spanned(span.end(parser.pos())))
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{
		Expr, Shorthand, TextDecorationLineValue, TextDecorationShorthand, TextDecorationStyleValue,
	};

	use crate::{Allocator, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn test_line_parses_underline() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "underline", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<TextDecorationLineValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 9),
				node: TextDecorationLineValue::Style {
					underline: true,
					overline: false,
					line_through: false,
					blink: false
				}
			}
		);
	}

	#[test]
	fn test_shorthand_parses_underline_dotted() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "underline dotted", ParserOptions::default());
		let node = TextDecorationShorthand {
			color: Shorthand::Implicit,
			line: Shorthand::Explicit(parser.boxup(Spanned {
				span: Span::new(0, 10),
				node: Expr::Literal(Spanned {
					span: Span::new(0, 10),
					node: TextDecorationLineValue::Style {
						underline: true,
						overline: false,
						line_through: false,
						blink: false,
					},
				}),
			})),
			style: Shorthand::Explicit(parser.boxup(Spanned {
				span: Span::new(10, 16),
				node: Expr::Literal(Spanned {
					span: Span::new(10, 16),
					node: TextDecorationStyleValue::Dotted,
				}),
			})),
		};
		let parser_return = parser.parse_entirely_with::<TextDecorationShorthand>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 16), node });
	}
}
