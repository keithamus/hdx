use hdx_ast::css::values::{LengthPercentage, MaxSizing, Sizing};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for Sizing {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let sizing = match parser.cur().kind {
			Kind::Ident => {
				let name = parser.expect_ident()?;
				match name {
					atom!("auto") => Sizing::Auto,
					atom!("max-content") => Sizing::MaxContent,
					atom!("min-content") => Sizing::MinContent,
					atom!("stretch") => Sizing::Stretch,
					atom!("fit-content") => Sizing::FitContent,
					_ => Err(diagnostics::UnexpectedIdent(name, parser.cur().span))?,
				}
			}
			Kind::Percentage | Kind::Dimension | Kind::Number => {
				Sizing::LengthPercentage(LengthPercentage::parse(parser)?)
			}
			Kind::Function => {
				parser.expect_function_of(atom!("fit-content"))?;
				let result = LengthPercentage::parse(parser)?;
				parser.expect(Kind::RightParen)?;
				Sizing::FitContentFunction(result)
			}
			_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
		};
		Ok(sizing.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> Parse<'a> for MaxSizing {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let sizing = match parser.cur().kind {
			Kind::Ident => {
				let name = parser.expect_ident()?;
				match name {
					atom!("none") => MaxSizing::None,
					atom!("max-content") => MaxSizing::MaxContent,
					atom!("min-content") => MaxSizing::MinContent,
					atom!("stretch") => MaxSizing::Stretch,
					atom!("fit-content") => MaxSizing::FitContent,
					_ => Err(diagnostics::UnexpectedIdent(name, parser.cur().span))?,
				}
			}
			Kind::Percentage | Kind::Dimension | Kind::Number => {
				MaxSizing::LengthPercentage(LengthPercentage::parse(parser)?)
			}
			Kind::Function => {
				let name = parser.expect_function_of(atom!("fit-content"))?;
				let result = LengthPercentage::parse(parser)?;
				parser.expect(Kind::RightParen)?;
				MaxSizing::FitContentFunction(result)
			}
			_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
		};
		Ok(sizing.spanned(span.until(parser.cur().span)))
	}
}

#[cfg(test)]
mod test {

	use hdx_ast::css::values::{LengthPercentage, Px};

	use super::*;
	use crate::{Allocator, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn parse_10px() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "10px", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Sizing>();
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 4),
				node: Sizing::LengthPercentage(Spanned {
					span: Span::new(0, 4),
					node: LengthPercentage::Px(Px(10.0))
				})
			}
		);
	}

	#[test]
	fn parse_0() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "0", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Sizing>();
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 1),
				node: Sizing::LengthPercentage(Spanned {
					span: Span::new(0, 1),
					node: LengthPercentage::Zero
				})
			}
		);
	}

	#[test]
	fn parse_auto() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "auto", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Sizing>();
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 4), node: Sizing::Auto });
	}
}
