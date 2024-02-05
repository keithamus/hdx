use hdx_ast::css::values::{
	AlignmentBaselineValue, BaselineShiftValue, BaselineSourceValue, Expr, LengthPercentage,
	LineHeightValue, MathExpr, Shorthand, VerticalAlignShorthand,
};

use crate::{atom, diagnostics, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for LineHeightValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => {
				if ident.eq_ignore_ascii_case(&atom!("normal")) {
					Ok(Self::Normal.spanned(parser.advance()))
				} else {
					Err(diagnostics::UnexpectedIdentSuggest(
						*ident,
						atom!("normal"),
						parser.span(),
					))?
				}
			}
			Token::Number(_, value) => Ok(Self::Number(*value).spanned(parser.advance())),
			_ => {
				let span = parser.span();
				let node = LengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.end(parser.pos())))
			}
		}
	}
}

impl<'a> Parse<'a> for BaselineShiftValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => match ident.as_ascii_lower() {
				atom!("sub") => Ok(Self::Sub.spanned(parser.advance())),
				atom!("super") => Ok(Self::Super.spanned(parser.advance())),
				atom!("top") => Ok(Self::Top.spanned(parser.advance())),
				atom!("center") => Ok(Self::Center.spanned(parser.advance())),
				atom!("bottom") => Ok(Self::Bottom.spanned(parser.advance())),
				_ => Err(diagnostics::UnexpectedIdent(*ident, parser.span()))?,
			},
			_ => {
				let span = parser.span();
				let node = LengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.end(parser.pos())))
			}
		}
	}
}

impl<'a> Parse<'a> for VerticalAlignShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut alignment_baseline = Shorthand::Implicit;
		let mut baseline_shift = Shorthand::Implicit;
		let mut baseline_source = match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lower() {
				atom!("first") => Shorthand::Explicit(
					parser.boxup(
						Expr::Literal(BaselineSourceValue::First.spanned(parser.span()))
							.spanned(parser.advance()),
					),
				),
				atom!("last") => Shorthand::Explicit(
					parser.boxup(
						Expr::Literal(BaselineSourceValue::Last.spanned(parser.span()))
							.spanned(parser.advance()),
					),
				),
			},
			_ => Shorthand::Implicit,
		};
		loop {
			if !matches!(
				parser.cur(),
				Token::Ident(_) | Token::Number(_, _) | Token::Dimension(_, _, _)
			) {
				break;
			}
			if alignment_baseline.is_explicit() && baseline_shift.is_explicit() {
				break;
			}
			if alignment_baseline.is_implicit() {
				let checkpoint = parser.checkpoint();
				if let Ok(expr) = Expr::<AlignmentBaselineValue>::parse(parser) {
					alignment_baseline = Shorthand::Explicit(parser.boxup(expr));
				} else {
					parser.rewind(checkpoint);
				}
			}
			if baseline_shift.is_implicit() {
				let checkpoint = parser.checkpoint();
				if let Ok(expr) = MathExpr::<BaselineShiftValue>::parse(parser) {
					baseline_shift = Shorthand::Explicit(parser.boxup(expr));
				} else {
					parser.rewind(checkpoint);
				}
			}
			if baseline_source.is_implicit() {
				let checkpoint = parser.checkpoint();
				if let Ok(expr) = Expr::<BaselineSourceValue>::parse(parser) {
					baseline_source = Shorthand::Explicit(parser.boxup(expr));
				} else {
					parser.rewind(checkpoint);
				}
			}
		}
		Ok(Self { baseline_source, alignment_baseline, baseline_shift }
			.spanned(span.end(parser.pos())))
	}
}
