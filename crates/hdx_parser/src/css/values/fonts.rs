use hdx_ast::css::values::{
	AbsoluteSize, Angle, FontFamilyValue, FontSizeValue, FontStyleValue, FontWeightValue, MathExpr,
	PositiveLengthPercentage, RelativeSize,
};

use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for FontWeightValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if ident == atom!("normal") {
					Ok(Self::Normal.spanned(span.end(parser.pos())))
				} else if ident == atom!("bold") {
					Ok(Self::Bold.spanned(span.end(parser.pos())))
				} else if ident == atom!("bolder") {
					Ok(Self::Bolder.spanned(span.end(parser.pos())))
				} else if ident == atom!("lighter") {
					Ok(Self::Lighter.spanned(span.end(parser.pos())))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, parser.span()))?
				}
			}
			Kind::Number => {
				let num = parser.cur().value.as_f32().unwrap();
				parser.advance();
				if (1.0..=1000.0).contains(&num) {
					Ok(Self::Number(num as u16).spanned(span.end(parser.pos())))
				} else {
					Err(diagnostics::NumberOutOfBounds(1.0, 1000.0, parser.span()))?
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
	}
}

impl<'a> Parse<'a> for FontSizeValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if ident == atom!("math") {
					Ok(Self::Math.spanned(span.end(parser.pos())))
				} else if let Some(val) = AbsoluteSize::from_atom(ident.clone()) {
					parser.advance();
					Ok(Self::Absolute(val).spanned(span.end(parser.pos())))
				} else if let Some(val) = RelativeSize::from_atom(ident.clone()) {
					parser.advance();
					Ok(Self::Relative(val).spanned(span.end(parser.pos())))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, parser.span()))?
				}
			}
			_ => {
				let node = PositiveLengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.end(parser.pos())))
			}
		}
	}
}

impl<'a> Parse<'a> for FontFamilyValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let mut ident = parser.expect_ident_cased()?;
				match ident.to_ascii_lowercase() {
					atom!("serif") => Ok(Self::Serif.spanned(span.end(parser.pos()))),
					atom!("sans-serif") => Ok(Self::SansSerif.spanned(span.end(parser.pos()))),
					atom!("cursive") => Ok(Self::Cursive.spanned(span.end(parser.pos()))),
					atom!("fantasy") => Ok(Self::Fantasy.spanned(span.end(parser.pos()))),
					atom!("monospace") => Ok(Self::Monospace.spanned(span.end(parser.pos()))),
					atom!("system-ui") => Ok(Self::SystemUi.spanned(span.end(parser.pos()))),
					atom!("emoji") => Ok(Self::Emoji.spanned(span.end(parser.pos()))),
					atom!("math") => Ok(Self::Math.spanned(span.end(parser.pos()))),
					atom!("fangsong") => Ok(Self::Fangsong.spanned(span.end(parser.pos()))),
					atom!("ui-serif") => Ok(Self::UiSerif.spanned(span.end(parser.pos()))),
					atom!("ui-sans-serif") => Ok(Self::UiSansSerif.spanned(span.end(parser.pos()))),
					atom!("ui-monospace") => Ok(Self::UiMonospace.spanned(span.end(parser.pos()))),
					atom!("ui-rounded") => Ok(Self::UiRounded.spanned(span.end(parser.pos()))),
					_ => {
						let mut name = String::new();
						loop {
							name.push_str(ident.as_ref());
							if !parser.at(Kind::Ident) {
								break;
							}
							name.push(' ');
							ident = parser.expect_ident_cased()?;
						}
						Ok(Self::Named(Atom::from(name)).spanned(span.end(parser.pos())))
					}
				}
			}
			Kind::String => {
				let string = parser.cur_atom().unwrap();
				parser.advance();
				Ok(Self::Named(string).spanned(span.end(parser.pos())))
			}
			_ => Err(diagnostics::Unexpected(parser.cur(), parser.span()))?,
		}
	}
}

impl<'a> Parse<'a> for FontStyleValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				match ident {
					atom!("normal") => Ok(Self::Normal.spanned(span.end(parser.pos()))),
					atom!("italic") => Ok(Self::Italic.spanned(span.end(parser.pos()))),
					atom!("oblique") => {
						if matches!(parser.cur(), Kind::Dimension | Kind::Number) {
							let degrees = MathExpr::<Angle>::parse(parser)?;
							Ok(Self::Oblique(degrees).spanned(span.end(parser.pos())))
						} else {
							Ok(Self::Oblique(Spanned::dummy(MathExpr::Literal(Spanned::dummy(
								Angle::Deg(14.0),
							))))
							.spanned(span.end(parser.pos())))
						}
					}
					_ => Err(diagnostics::UnexpectedIdent(ident, parser.span()))?,
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{Angle, FontFamilyValue, FontStyleValue, FontWeightValue, MathExpr};

	use crate::{Allocator, Atom, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn test_font_weight_numeric() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "326", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<FontWeightValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 3), node: FontWeightValue::Number(326) });
	}

	#[test]
	fn test_basic_named() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "Roboto", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<FontFamilyValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 6), node: FontFamilyValue::Named(Atom::from("Roboto")) }
		);
	}

	#[test]
	fn test_multi_ident_named() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "Gill Sans Semibold", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<FontFamilyValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 18),
				node: FontFamilyValue::Named(Atom::from("Gill Sans Semibold"))
			}
		);
	}

	#[test]
	fn test_multi_string_named() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "'Gill Sans Semibold'", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<FontFamilyValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 20),
				node: FontFamilyValue::Named(Atom::from("Gill Sans Semibold"))
			}
		);
	}

	#[test]
	fn test_font_style_oblique_deg() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "oblique 30deg", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<FontStyleValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 13),
				node: FontStyleValue::Oblique(Spanned {
					span: Span::new(8, 13),
					node: (MathExpr::Literal(Spanned {
						span: Span::new(8, 13),
						node: Angle::Deg(30.0)
					}))
				})
			}
		);
	}
}
