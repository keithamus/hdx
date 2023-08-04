use hdx_ast::css::values::{
	BorderShorthand, ColorValue, Expr, Length, LineStyle, LineWidth, MathExpr, Shorthand,
};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for BorderShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut line_width = Shorthand::Implicit;
		let mut line_style = Shorthand::Implicit;
		let mut color = Shorthand::Implicit;
		loop {
			match parser.cur().kind {
				Kind::Ident => {
					let ident = parser.cur_atom().unwrap();
					if line_style.is_implicit()
						&& matches!(
							ident,
							atom!("none")
								| atom!("hidden") | atom!("dotted")
								| atom!("dashed") | atom!("solid")
								| atom!("double") | atom!("groove")
								| atom!("ridge") | atom!("inset")
								| atom!("outset")
						) {
						let node = Expr::<LineStyle>::parse(parser)?;
						line_style = Shorthand::Explicit(parser.boxup(node));
					} else if line_width.is_implicit()
						&& matches!(ident, atom!("thin") | atom!("medium") | atom!("thick"))
					{
						let node = MathExpr::<LineWidth>::parse(parser)?;
						line_width = Shorthand::Explicit(parser.boxup(node));
					} else if color.is_implicit() {
						let node = MathExpr::<ColorValue>::parse(parser)?;
						color = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(ident.clone(), parser.cur().span))?
					}
				}
				Kind::Semicolon | Kind::Comma | Kind::Eof => {
					break;
				}
				Kind::Dimension => {
					if line_width.is_implicit() {
						let node = MathExpr::<LineWidth>::parse(parser)?;
						line_width = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::Unexpected(Kind::Dimension, parser.cur().span))?
					}
				}
				k => {
					let checkpoint = parser.checkpoint();
					if line_width.is_implicit() {
						let node = MathExpr::<LineWidth>::parse(parser);
						match node {
							Ok(node) => {
								line_width = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					let checkpoint = parser.checkpoint();
					if color.is_implicit() {
						let node = MathExpr::<ColorValue>::parse(parser);
						match node {
							Ok(node) => {
								color = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					Err(diagnostics::Unexpected(k, parser.cur().span))?
				}
			}
			if color.is_explicit() && line_style.is_explicit() && line_width.is_explicit() {
				break;
			}
		}
		Ok(Self { color, line_style, line_width }.spanned(span.up_to(&parser.cur().span)))
	}
}

impl<'a> Parse<'a> for LineWidth {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Number | Kind::Dimension => {
				Ok(Self::Length(Length::parse(parser)?).spanned(span.up_to(&parser.cur().span)))
			}
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				match ident {
					atom!("thin") => Ok(Self::Thin.spanned(span)),
					atom!("medium") => Ok(Self::Medium.spanned(span)),
					atom!("thick") => Ok(Self::Thick.spanned(span)),
					_ => Err(diagnostics::UnexpectedIdent(ident, span))?,
				}
			}
			k => Err(diagnostics::Unexpected(k, span))?,
		}
	}
}

#[cfg(test)]
mod test {

	use hdx_ast::css::values::{
		BorderShorthand, ColorValue, Expr, Length, LineStyle, LineWidth, MathExpr, NamedColor, Px,
		Shorthand,
	};

	use crate::{Allocator, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn parses_border_1px_solid_red() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "1px solid red", ParserOptions::default());
		let expected = Spanned {
			span: Span::new(0, 13),
			node: BorderShorthand {
				line_width: Shorthand::Explicit(parser.boxup(Spanned {
					span: Span::new(0, 4),
					node: MathExpr::Literal(Spanned {
						span: Span::new(0, 4),
						node: LineWidth::Length(Spanned {
							span: Span::new(0, 4),
							node: Length::Px(Px(1.0)),
						}),
					}),
				})),
				line_style: Shorthand::Explicit(parser.boxup(Spanned {
					span: Span::new(4, 10),
					node: Expr::Literal(Spanned { span: Span::new(4, 10), node: LineStyle::Solid }),
				})),
				color: Shorthand::Explicit(parser.boxup(Spanned {
					span: Span::new(10, 13),
					node: MathExpr::Literal(Spanned {
						span: Span::new(10, 13),
						node: ColorValue::Named(NamedColor::Red),
					}),
				})),
			},
		};
		let parser_return = parser.parse_entirely_with::<BorderShorthand>();
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
