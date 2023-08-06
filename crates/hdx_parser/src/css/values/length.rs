use hdx_ast::css::values::{lengths::*, Percentage};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

// https://drafts.csswg.org/css-values-4/#lengths
impl<'a> Parse<'a> for Length {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Dimension => {
				let (value, atom) = parser.expect_dimension()?;
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, span))?
				}
			}
			Kind::Number => {
				let value = parser.expect_number()?;
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("px"),
						parser.cur().span,
					))?
				}
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			k => Err(diagnostics::Unexpected(k, span))?,
		}
	}
}

// https://drafts.csswg.org/css-values-4/#lengths
impl<'a> Parse<'a> for PositiveLength {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Dimension => {
				let (value, atom) = parser.expect_dimension_gte(0.0)?;
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, span))?
				}
			}
			Kind::Number => {
				let value = parser.expect_number()?;
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(atom!("px"), span))?
				}
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			k => Err(diagnostics::Unexpected(k, span))?,
		}
	}
}

// https://drafts.csswg.org/css-values-4/#lengths
impl<'a> Parse<'a> for PositiveLengthPercentageOrNormal {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				parser.expect_ident_of(atom!("normal"))?;
				Ok(Self::Normal.spanned(span.until(parser.cur().span)))
			}
			Kind::Dimension => {
				let (value, atom) = parser.expect_dimension_gte(0.0)?;
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, span))?
				}
			}
			Kind::Percentage => {
				let value = parser.expect_percentage_gte(0.0)?;
				Ok(Self::Percentage(Percentage(value)).spanned(span.until(parser.cur().span)))
			}
			Kind::Number => {
				let value = parser.expect_number()?;
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(atom!("px"), span))?
				}
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			k => Err(diagnostics::Unexpected(k, span))?,
		}
	}
}

// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
impl<'a> Parse<'a> for LengthPercentage {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Dimension => {
				let value = parser.cur().value.as_f32().unwrap();
				let atom = parser.cur().value.as_atom().unwrap();
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					parser.advance();
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, parser.cur().span))?
				}
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("px"),
						parser.cur().span,
					))?
				}
				parser.advance();
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Percentage(Percentage(value)).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
impl<'a> Parse<'a> for LengthPercentageOrNormal {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				if parser.cur_atom_lower().unwrap() == atom!("normal") {
					parser.advance();
					Ok(Self::Normal.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(
						parser.cur_atom_lower().unwrap(),
						parser.cur().span,
					))?
				}
			}
			Kind::Dimension => {
				let value = parser.cur().value.as_f32().unwrap();
				let atom = parser.cur().value.as_atom().unwrap();
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					parser.advance();
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, parser.cur().span))?
				}
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("px"),
						parser.cur().span,
					))?
				}
				parser.advance();
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Percentage(Percentage(value)).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
impl<'a> Parse<'a> for PositiveLengthPercentage {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Dimension => {
				let value = parser.cur().value.as_f32().unwrap();
				if value < 0.0 {
					Err(diagnostics::NumberOutOfBounds(value, 0.0, parser.cur().span))?;
				}
				let atom = parser.cur().value.as_atom().unwrap();
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					parser.advance();
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, parser.cur().span))?
				}
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("px"),
						parser.cur().span,
					))?
				}
				parser.advance();
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				if value < 0.0 {
					Err(diagnostics::NumberOutOfBounds(value, 0.0, parser.cur().span))?;
				}
				parser.advance();
				Ok(Self::Percentage(Percentage(value)).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

impl<'a> Parse<'a> for LengthPercentageOrAuto {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				if parser.cur_atom_lower().unwrap() == atom!("auto") {
					parser.advance();
					Ok(Self::Auto.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(
						parser.cur_atom_lower().unwrap(),
						parser.cur().span,
					))?
				}
			}
			Kind::Dimension => {
				let value = parser.cur().value.as_f32().unwrap();
				let atom = parser.cur().value.as_atom().unwrap();
				if let Some(unit) = Self::from_f32_and_atom(value, atom.clone()) {
					parser.advance();
					Ok(unit.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedDimension(atom, parser.cur().span))?
				}
			}
			Kind::Number => {
				let value = parser.cur().value.as_f32().unwrap();
				if value != 0.0 {
					Err(diagnostics::DisallowedValueWithoutDimension(
						atom!("px"),
						parser.cur().span,
					))?
				}
				parser.advance();
				Ok(Self::Zero.spanned(span.until(parser.cur().span)))
			}
			Kind::Percentage => {
				let value = parser.cur().value.as_f32().unwrap();
				parser.advance();
				Ok(Self::Percentage(Percentage(value)).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{LengthPercentage, Percentage, Px};
	use oxc_allocator::Allocator;

	use crate::{Parser, ParserOptions, Span, Spanned};

	#[test]
	fn parses_0() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "0", ParserOptions::default());
		let parser_return = parser.parse_with::<LengthPercentage>();
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 1), node: LengthPercentage::Zero });
	}

	#[test]
	fn parses_3pc() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "3%", ParserOptions::default());
		let parser_return = parser.parse_with::<LengthPercentage>();
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 2), node: LengthPercentage::Percentage(Percentage(3.0)) }
		);
	}

	#[test]
	fn parses_10dot4px() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "10.4px", ParserOptions::default());
		let parser_return = parser.parse_with::<LengthPercentage>();
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 6), node: LengthPercentage::Px(Px(10.4)) });
	}
}
