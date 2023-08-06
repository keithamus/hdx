use hdx_ast::css::values::{
	Expr, ExprList, ExprListItem, GlobalValue, MathExpr, MathExprList, MathExprListItem, MathFunc,
	Reference,
};

use crate::{atom, diagnostics, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a, T> Parse<'a> for Expr<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = GlobalValue::from_atom(parser.cur().as_atom().unwrap()) {
					parser.advance();
					Self::GlobalValue(val).spanned(span.until(parser.cur().span))
				} else {
					Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span))
				}
			}
			Kind::Function => match parser.cur().as_atom().unwrap() {
				atom!("var") | atom!("env") => {
                    let node = Reference::parse(parser)?;
                    Self::Reference(node).spanned(span.until(parser.cur().span))
                }
                atom!("calc") /*TODO! ...*/ => {
                    Err(diagnostics::DisallowedMathFunction(parser.cur().as_atom().unwrap(), parser.cur().span))?
                },
				_ => {
                    let node = T::parse(parser)?;
                    Self::Literal(node).spanned(span.until(parser.cur().span))
                }
			},
			_ => {
				let node = T::parse(parser)?;
				Self::Literal(node).spanned(span.until(parser.cur().span))
			}
		})
	}
}

impl<'a, T> Parse<'a> for MathExpr<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = GlobalValue::from_atom(parser.cur().as_atom().unwrap()) {
					parser.advance();
					Self::GlobalValue(val).spanned(span.until(parser.cur().span))
				} else {
					let node = T::parse(parser)?;
					Self::Literal(node).spanned(span.until(parser.cur().span))
				}
			}
			Kind::Function => {
				match parser.cur().value.as_atom().unwrap() {
                    atom!("var") | atom!("env") => {
                        let node = Reference::parse(parser)?;
                        Self::Reference(node).spanned(span.until(parser.cur().span))
                    },
                    atom!("calc") /*TODO! ...*/ => {
                        let node = MathFunc::parse(parser)?;
                        Self::Math(node).spanned(span.until(parser.cur().span))
                    },
                    _ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span))
                }
			}
			_ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
		})
	}
}

impl<'a, T> Parse<'a> for ExprList<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = GlobalValue::from_atom(parser.cur().as_atom().unwrap()) {
					parser.advance();
					Self::GlobalValue(val).spanned(span.until(parser.cur().span))
				} else {
					Self::Values(parser.parse_comma_list_of::<ExprListItem<T>>()?)
						.spanned(span.until(parser.cur().span))
				}
			}
			_ => Self::Values(parser.parse_comma_list_of::<ExprListItem<T>>()?)
				.spanned(span.until(parser.cur().span)),
		})
	}
}

impl<'a, T> Parse<'a> for MathExprList<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => {
				if let Some(val) = GlobalValue::from_atom(parser.cur().as_atom().unwrap()) {
					parser.advance();
					Self::GlobalValue(val).spanned(span.until(parser.cur().span))
				} else {
					Self::Values(parser.parse_comma_list_of::<MathExprListItem<T>>()?)
						.spanned(span.until(parser.cur().span))
				}
			}
			_ => Self::Values(parser.parse_comma_list_of::<MathExprListItem<T>>()?)
				.spanned(span.until(parser.cur().span)),
		})
	}
}

impl<'a, T> Parse<'a> for ExprListItem<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
			Kind::Function => match parser.cur().as_atom().unwrap() {
				atom!("var") | atom!("env") => Self::Reference(Reference::parse(parser)?).spanned(span.until(parser.cur().span)),
                atom!("calc") /*TODO! ...*/ => {
                    Err(diagnostics::DisallowedMathFunction(parser.cur().as_atom().unwrap(), parser.cur().span))?
                },
				_ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
			},
			_ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
		})
	}
}

impl<'a, T> Parse<'a> for MathExprListItem<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(match parser.cur().kind {
			Kind::Ident => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
			Kind::Function => {
				match parser.cur().as_atom().unwrap() {
                    atom!("var") | atom!("env") => {
                        Self::Reference(Reference::parse(parser)?).spanned(span.until(parser.cur().span))
                    },
                    atom!("calc") /*TODO! ...*/ => {
                        Self::Math(MathFunc::parse(parser)?).spanned(span.until(parser.cur().span))
                    },
                    _ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span))
                }
			}
			_ => Self::Literal(T::parse(parser)?).spanned(span.until(parser.cur().span)),
		})
	}
}

impl<'a> Parse<'a> for MathFunc<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		parser.expect(Kind::Function)?;
		todo!()
	}
}

impl<'a, T> Parse<'a> for Reference<'a, T>
where
	T: Parse<'a>,
{
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let ident = parser.expect_function()?;
		Ok(match ident {
			atom!("var") => {
				let name = parser.expect_ident()?;
				let mut inner = None;
				if parser.at(Kind::Comma) {
					parser.advance();
					inner = Some(T::parse(parser)?)
				}
				parser.expect(Kind::RightParen)?;
				Self::Var(name, parser.boxup(inner)).spanned(span.until(parser.cur().span))
			}
			atom!("env") => {
				let name = parser.expect_ident()?;
				let mut inner = None;
				if parser.at(Kind::Comma) {
					parser.advance();
					inner = Some(T::parse(parser)?)
				}
				parser.expect(Kind::RightParen)?;
				Self::Env(name, parser.boxup(inner)).spanned(span.until(parser.cur().span))
			}
			_ => Err(diagnostics::UnexpectedFunction(ident, parser.cur().span))?,
		})
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{ColorValue, Expr, GlobalValue, Length, NamedColor, Px, Reference};

	use crate::{Allocator, Atom, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn test_expr_basic_inherit() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "inherit", ParserOptions::default());
		let node = Expr::<Length>::GlobalValue(GlobalValue::Inherit);
		let parser_return = parser.parse_entirely_with::<Expr<Length>>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 7), node });
	}

	#[test]
	fn test_expr_basic_var() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "var(--foo)", ParserOptions::default());
		let fallback = None;
		let node = Expr::<Length>::Reference(Spanned {
			span: Span::new(0, 10),
			node: Reference::Var(Atom::from("--foo"), parser.boxup(fallback)),
		});
		let parser_return = parser.parse_entirely_with::<Expr<Length>>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 10), node });
	}

	#[test]
	fn test_expr_var_colorvalue_with_fallback() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "var(--foo, blue)", ParserOptions::default());
		let fallback = Some(Spanned {
			span: Span::new(11, 15),
			node: Expr::Literal(Spanned {
				span: Span::new(11, 15),
				node: ColorValue::Named(NamedColor::Blue),
			}),
		});
		let node = Expr::<ColorValue>::Reference(Spanned {
			span: Span::new(0, 16),
			node: Reference::Var(Atom::from("--foo"), parser.boxup(fallback)),
		});
		let parser_return = parser.parse_entirely_with::<Expr<ColorValue>>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 16), node });
	}

	#[test]
	fn test_expr_var_with_fallback() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "var(--foo, 8px)", ParserOptions::default());
		let fallback = Some(Spanned {
			span: Span::new(11, 14),
			node: Expr::Literal(Spanned { span: Span::new(11, 14), node: Length::Px(Px(8.0)) }),
		});
		let node = Expr::<Length>::Reference(Spanned {
			span: Span::new(0, 15),
			node: Reference::Var(Atom::from("--foo"), parser.boxup(fallback)),
		});
		let parser_return = parser.parse_entirely_with::<Expr<Length>>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 15), node });
	}

	#[test]
	fn test_expr_env_with_var_with_fallback() {
		let allocator = Allocator::default();
		let parser =
			Parser::new(&allocator, "env(--bar, var(--foo, 8px))", ParserOptions::default());
		let var_fallback = Some(Spanned {
			span: Span::new(22, 25),
			node: Expr::Literal(Spanned { span: Span::new(22, 25), node: Length::Px(Px(8.0)) }),
		});
		let env_fallback = Some(Spanned {
			span: Span::new(11, 26),
			node: Expr::Reference(Spanned {
				span: Span::new(11, 26),
				node: Reference::Var(Atom::from("--foo"), parser.boxup(var_fallback)),
			}),
		});
		let node = Expr::<Length>::Reference(Spanned {
			span: Span::new(0, 27),
			node: Reference::Env(Atom::from("--bar"), parser.boxup(env_fallback)),
		});
		let parser_return = parser.parse_entirely_with::<Expr<Length>>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 27), node });
	}
}
