use hdx_ast::css::values::{
	AbsoluteSize, Angle, FontFamilyValue, FontSizeValue, FontStyleValue, FontWeightValue, MathExpr,
	PositiveLengthPercentage, RelativeSize,
};

use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for FontWeightValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if ident == atom!("normal") {
					Ok(Self::Normal.spanned(span.up_to(&parser.cur().span)))
				} else if ident == atom!("bold") {
					Ok(Self::Bold.spanned(span.up_to(&parser.cur().span)))
				} else if ident == atom!("bolder") {
					Ok(Self::Bolder.spanned(span.up_to(&parser.cur().span)))
				} else if ident == atom!("lighter") {
					Ok(Self::Lighter.spanned(span.up_to(&parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, parser.cur().span))?
				}
			}
			Kind::Number => {
				let num = parser.cur().value.as_f32().unwrap();
				parser.advance();
				if (1.0..=1000.0).contains(&num) {
					Ok(Self::Number(num as u16).spanned(span.up_to(&parser.cur().span)))
				} else {
					Err(diagnostics::NumberOutOfBounds(1.0, 1000.0, parser.cur().span))?
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
	}
}

impl<'a> Parse<'a> for FontSizeValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if ident == atom!("math") {
					Ok(Self::Math.spanned(span.up_to(&parser.cur().span)))
				} else if let Some(val) = AbsoluteSize::from_atom(ident.clone()) {
					parser.advance();
					Ok(Self::Absolute(val).spanned(span.up_to(&parser.cur().span)))
				} else if let Some(val) = RelativeSize::from_atom(ident.clone()) {
					parser.advance();
					Ok(Self::Relative(val).spanned(span.up_to(&parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, parser.cur().span))?
				}
			}
			_ => {
				let node = PositiveLengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.up_to(&parser.cur().span)))
			}
		}
	}
}

impl<'a> Parse<'a> for FontFamilyValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let mut ident = parser.expect_ident_cased()?;
				match ident.to_ascii_lowercase() {
					atom!("serif") => Ok(Self::Serif.spanned(span.up_to(&parser.cur().span))),
					atom!("sans-serif") => {
						Ok(Self::SansSerif.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("cursive") => Ok(Self::Cursive.spanned(span.up_to(&parser.cur().span))),
					atom!("fantasy") => Ok(Self::Fantasy.spanned(span.up_to(&parser.cur().span))),
					atom!("monospace") => {
						Ok(Self::Monospace.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("system-ui") => {
						Ok(Self::SystemUi.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("emoji") => Ok(Self::Emoji.spanned(span.up_to(&parser.cur().span))),
					atom!("math") => Ok(Self::Math.spanned(span.up_to(&parser.cur().span))),
					atom!("fangsong") => Ok(Self::Fangsong.spanned(span.up_to(&parser.cur().span))),
					atom!("ui-serif") => Ok(Self::UiSerif.spanned(span.up_to(&parser.cur().span))),
					atom!("ui-sans-serif") => {
						Ok(Self::UiSansSerif.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("ui-monospace") => {
						Ok(Self::UiMonospace.spanned(span.up_to(&parser.cur().span)))
					}
					atom!("ui-rounded") => {
						Ok(Self::UiRounded.spanned(span.up_to(&parser.cur().span)))
					}
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
						Ok(Self::Named(Atom::from(name)).spanned(span.up_to(&parser.cur().span)))
					}
				}
			}
			Kind::String => {
				let string = parser.cur_atom().unwrap();
				parser.advance();
				Ok(Self::Named(string).spanned(span.up_to(&parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

impl<'a> Parse<'a> for FontStyleValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				match ident {
					atom!("normal") => Ok(Self::Normal.spanned(span.up_to(&parser.cur().span))),
					atom!("italic") => Ok(Self::Italic.spanned(span.up_to(&parser.cur().span))),
					atom!("oblique") => {
						if matches!(parser.cur().kind, Kind::Dimension | Kind::Number) {
							let degrees = MathExpr::<Angle>::parse(parser)?;
							Ok(Self::Oblique(degrees).spanned(span.up_to(&parser.cur().span)))
						} else {
							Ok(Self::Oblique(Spanned::dummy(MathExpr::Literal(Spanned::dummy(
								Angle::Deg(14.0),
							))))
							.spanned(span.up_to(&parser.cur().span)))
						}
					}
					_ => Err(diagnostics::UnexpectedIdent(ident, parser.cur().span))?,
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
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
