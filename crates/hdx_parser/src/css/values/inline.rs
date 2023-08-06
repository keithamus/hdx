use hdx_ast::css::values::{
	AlignmentBaselineValue, BaselineShiftValue, BaselineSourceValue, Expr, LengthPercentage,
	LineHeightValue, MathExpr, Shorthand, VerticalAlignShorthand,
};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for LineHeightValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if ident == atom!("normal") {
					Ok(Self::Normal.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(ident, parser.cur().span))?
				}
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Number(value).spanned(span.until(parser.cur().span)))
			}
			_ => {
				let node = LengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.until(parser.cur().span)))
			}
		}
	}
}

impl<'a> Parse<'a> for BaselineShiftValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let span = parser.cur().span;
				let ident = parser.expect_ident()?;
				match ident {
					atom!("sub") => Ok(Self::Sub.spanned(span.until(parser.cur().span))),
					atom!("super") => Ok(Self::Super.spanned(span.until(parser.cur().span))),
					atom!("top") => Ok(Self::Top.spanned(span.until(parser.cur().span))),
					atom!("center") => Ok(Self::Center.spanned(span.until(parser.cur().span))),
					atom!("bottom") => Ok(Self::Bottom.spanned(span.until(parser.cur().span))),
					_ => Err(diagnostics::UnexpectedIdent(ident, span))?,
				}
			}
			_ => {
				let node = LengthPercentage::parse(parser)?;
				Ok(Self::LengthPercentage(node).spanned(span.until(parser.cur().span)))
			}
		}
	}
}

impl<'a> Parse<'a> for VerticalAlignShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut baseline_source = Shorthand::Implicit;
		let mut alignment_baseline = Shorthand::Implicit;
		let mut baseline_shift = Shorthand::Implicit;
		if parser.at(Kind::Ident) {
			match parser.cur().as_atom().unwrap() {
				atom!("first") => {
					baseline_source = Shorthand::Explicit(
						parser.boxup(
							Expr::Literal(
								BaselineSourceValue::First.spanned(span.until(parser.cur().span)),
							)
							.spanned(span.until(parser.cur().span)),
						),
					);
					parser.advance();
				}
				atom!("last") => {
					baseline_source = Shorthand::Explicit(
						parser.boxup(
							Expr::Literal(
								BaselineSourceValue::Last.spanned(span.until(parser.cur().span)),
							)
							.spanned(span.until(parser.cur().span)),
						),
					);
					parser.advance();
				}
				_ => {}
			}
		}
		loop {
			if !matches!(
				parser.cur().kind,
				Kind::Ident | Kind::Number | Kind::Percentage | Kind::Dimension
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
			.spanned(span.until(parser.cur().span)))
	}
}
