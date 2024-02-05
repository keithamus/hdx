use hdx_ast::css::values::{shorthand::*, Shorthand};

use crate::{Parse, Parser, Result, Spanned, Token};

impl<'a, T: Parse<'a>> Parse<'a> for BoxShorthand<'a, T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut top = Shorthand::Implicit;
		let mut right = Shorthand::Implicit;
		let mut bottom = Shorthand::Implicit;
		let mut left = Shorthand::Implicit;
		while matches!(
			parser.cur(),
			Token::Dimension(_, _, _) | Token::Number(_, _) | Token::Ident(_) | Token::Function(_)
		) {
			let parsed = T::parse(parser)?;
			if top.is_implicit() {
				top = Shorthand::Explicit(parser.boxup(parsed));
			} else if right.is_implicit() {
				right = Shorthand::Explicit(parser.boxup(parsed));
			} else if bottom.is_implicit() {
				bottom = Shorthand::Explicit(parser.boxup(parsed));
			} else if left.is_implicit() {
				left = Shorthand::Explicit(parser.boxup(parsed));
				break;
			}
		}
		Ok(Self { top, right, bottom, left }.spanned(span.end(parser.pos())))
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for XYShorthand<'a, T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let node = T::parse(parser)?;
		let x = Shorthand::Explicit(parser.boxup(node));
		match parser.cur() {
			Token::Ident(_) | Token::Function(_) => {
				let node = T::parse(parser)?;
				Ok(Self { x, y: Shorthand::Explicit(parser.boxup(node)) }
					.spanned(span.end(parser.pos())))
			}
			_ => Ok(Self { x, y: Shorthand::Implicit }.spanned(span.end(parser.pos()))),
		}
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for DoubleShorthand<'a, T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let node = T::parse(parser)?;
		let first = Shorthand::Explicit(parser.boxup(node));
		match parser.cur() {
			Token::Ident(_) | Token::Function(_) => {
				let node = T::parse(parser)?;
				Ok(Self(first, Shorthand::Explicit(parser.boxup(node)))
					.spanned(span.end(parser.pos())))
			}
			_ => Ok(Self(first, Shorthand::Implicit).spanned(span.end(parser.pos()))),
		}
	}
}
