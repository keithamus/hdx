use hdx_ast::css::values::{
	color::{ColorValue, NamedColor, NumberPercentageOrNone, RGB},
	MathExpr,
};

use crate::{atom, diagnostics, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for ColorValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			// https://drafts.csswg.org/css-color/#hex-notation
			Kind::Hash => {
				let hash = parser.expect_hash()?;
				if let Some(hex) = ColorValue::from_hex(hash.as_ref()) {
					Ok(hex.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::BadHexColor(hash.clone(), span))?
				}
			}
			Kind::Ident => {
				let name = parser.expect_ident()?;
				match name {
					atom!("transparent") => {
						Ok(ColorValue::Transparent.spanned(span.until(parser.cur().span)))
					}
					_ => match NamedColor::from_atom(name.clone()) {
						Some(n) => Ok(ColorValue::Named(n).spanned(span.until(parser.cur().span))),
						None => Err(diagnostics::UnknownColor(name, span))?,
					},
				}
			}
			Kind::Function => {
				let name = parser.cur().as_atom_lower().unwrap();
				match name {
					atom!("rgb") | atom!("rgba") => {
						let node = RGB::parse(parser)?;
						Ok(ColorValue::RGB(parser.boxup(node))
							.spanned(span.until(parser.cur().span)))
					}
					_ => Err(diagnostics::Unimplemented(span))?,
				}
			}
			_ => Err(diagnostics::Unimplemented(span))?,
		}
	}
}

impl<'a> Parse<'a> for RGB<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let ident = parser.expect_function()?;
		let mut legacy = false;
		let r = MathExpr::<NumberPercentageOrNone>::parse(parser)?;
		if parser.at(Kind::Comma) {
			legacy = true;
			parser.advance();
		}
		let g = MathExpr::<NumberPercentageOrNone>::parse(parser)?;
		if legacy {
			parser.expect(Kind::Comma)?
		}
		let b = MathExpr::<NumberPercentageOrNone>::parse(parser)?;
		let mut alpha =
			Spanned::dummy(MathExpr::Literal(Spanned::dummy(NumberPercentageOrNone::Number(1.0))));
		if ident == atom!("rgba") {
			if legacy {
				parser.expect(Kind::Comma)?
			} else {
				parser.expect_delim_of('/')?
			}
			alpha = MathExpr::<NumberPercentageOrNone>::parse(parser)?;
		}
		parser.expect(Kind::RightParen)?;
		Ok(Self { r, g, b, alpha }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> Parse<'a> for NumberPercentageOrNone {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Number => {
				Ok(Self::Number(parser.expect_number()?).spanned(span.until(parser.cur().span)))
			}
			Kind::Percentage => Ok(Self::Percentage(parser.expect_percentage()?)
				.spanned(span.until(parser.cur().span))),
			Kind::Ident => match parser.expect_ident()? {
				atom!("none") => {
					parser.advance();
					Ok(Self::None.spanned(span.until(parser.cur().span)))
				}
				_ => Err(diagnostics::Unimplemented(span))?,
			},
			_ => Err(diagnostics::Unimplemented(span))?,
		}
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{
		color::{ColorValue, NamedColor, NumberPercentageOrNone, RGB},
		MathExpr,
	};
	use oxc_allocator::Allocator;

	use crate::{Parser, ParserOptions, Span, Spanned};

	#[test]
	fn parses_named_black() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "black", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<ColorValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 5), node: ColorValue::Named(NamedColor::Black) }
		);
	}

	#[test]
	fn parses_named_rebeccapurple() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "rebeccapurple", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<ColorValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 13), node: ColorValue::Named(NamedColor::Rebeccapurple) }
		);
	}

	#[test]
	fn parses_hex() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "#abc", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<ColorValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 4), node: ColorValue::Hex(2864434431) });
	}

	#[test]
	fn parses_rgb() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "rgb(0, 128, 255)", ParserOptions::default());
		let color = parser.boxup(Spanned {
			span: Span::new(0, 16),
			node: RGB {
				r: Spanned {
					span: Span::new(4, 5),
					node: MathExpr::Literal(Spanned {
						span: Span::new(4, 5),
						node: NumberPercentageOrNone::Number(0.0),
					}),
				},
				g: Spanned {
					span: Span::new(7, 10),
					node: MathExpr::Literal(Spanned {
						span: Span::new(7, 10),
						node: NumberPercentageOrNone::Number(128.0),
					}),
				},
				b: Spanned {
					span: Span::new(12, 15),
					node: MathExpr::Literal(Spanned {
						span: Span::new(12, 15),
						node: NumberPercentageOrNone::Number(255.0),
					}),
				},
				alpha: Spanned::dummy(MathExpr::Literal(Spanned::dummy(
					NumberPercentageOrNone::Number(1.0),
				))),
			},
		});
		let parser_return = parser.parse_entirely_with::<ColorValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 16), node: ColorValue::RGB(color) })
	}
}
