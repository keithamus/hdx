use hdx_ast::css::values::display::{DisplayInside, DisplayMarker, DisplayOutside, DisplayValue};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for DisplayValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let first = parser.expect_ident()?;
		let mut second_option = None;
		let mut third_option = None;
		if parser.at(Kind::Ident) {
			second_option = Some(parser.expect_ident()?);
			if parser.at(Kind::Ident) {
				third_option = Some(parser.expect_ident()?);
			}
		}
		let mut inside = DisplayInside::default();
		let mut outside = DisplayOutside::default();
		match (second_option, third_option) {
			// <display-outside> && [ flow | flow-root ] && list-item
			(Some(second), Some(third)) => {
				if let Some(display) = DisplayOutside::from_atom(first.clone()) {
					outside = display;
				} else {
					Err(diagnostics::UnexpectedIdent(first, parser.cur().span))?;
				}
				// second must be flow or flow-root
				if second != atom!("flow") && second != atom!("flow-root") {
					Err(diagnostics::UnexpectedIdent(second.clone(), parser.cur().span))?;
				}
				inside = DisplayInside::from_atom(second).unwrap();
				if let Some(marker) = DisplayMarker::from_atom(third.clone()) {
					Ok(DisplayValue::PairAndMarker(outside, inside, marker)
						.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(third, parser.cur().span).into())
				}
			}
			// [ <display-outside> || <display-inside> ]
			(Some(second), None) => {
				if let Some(display) = DisplayOutside::from_atom(first.clone()) {
					outside = display;
					if let Some(display) = DisplayMarker::from_atom(second.clone()) {
						Ok(DisplayValue::PairAndMarker(outside, inside, display)
							.spanned(span.until(parser.cur().span)))
					} else if let Some(display) = DisplayInside::from_atom(second.clone()) {
						Ok(DisplayValue::Pair(outside, display)
							.spanned(span.until(parser.cur().span)))
					} else {
						Err(diagnostics::UnexpectedIdent(second, parser.cur().span).into())
					}
				} else if let Some(display) = DisplayInside::from_atom(first.clone()) {
					inside = display;
					if let Some(display) = DisplayOutside::from_atom(second.clone()) {
						Ok(DisplayValue::Pair(display, inside)
							.spanned(span.until(parser.cur().span)))
					} else {
						Err(diagnostics::UnexpectedIdent(second, parser.cur().span).into())
					}
				} else {
					Err(diagnostics::UnexpectedIdent(first, parser.cur().span).into())
				}
			}
			// <display-listitem> | <display-internal> | <display-box> | <display-legacy>
			_ => {
				if let Some(display) = DisplayValue::from_atom(first.clone()) {
					Ok(display.spanned(span.until(parser.cur().span)))
				} else {
					Err(diagnostics::UnexpectedIdent(first, parser.cur().span).into())
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::values::{DisplayInside, DisplayMarker, DisplayOutside, DisplayValue};
	use oxc_allocator::Allocator;

	use crate::{Parser, ParserOptions, Span, Spanned};

	#[test]
	fn parses_block() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "block", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 5),
				node: DisplayValue::Pair(DisplayOutside::Block, DisplayInside::Implicit)
			}
		);
	}

	#[test]
	fn parses_flow_root() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "flow-root", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 9),
				node: DisplayValue::Pair(DisplayOutside::Block, DisplayInside::FlowRoot)
			}
		);
	}

	#[test]
	fn parses_inline() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "inline", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 6),
				node: DisplayValue::Pair(DisplayOutside::Inline, DisplayInside::Implicit)
			}
		);
	}

	#[test]
	fn parses_inline_block() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "inline-block", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 12), node: DisplayValue::InlineBlock });
	}

	#[test]
	fn parses_run_in() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "run-in", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 6),
				node: DisplayValue::Pair(DisplayOutside::RunIn, DisplayInside::Implicit)
			}
		);
	}

	#[test]
	fn parses_list_item() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "list-item", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 9),
				node: DisplayValue::PairAndMarker(
					DisplayOutside::Implicit,
					DisplayInside::Implicit,
					DisplayMarker::ListItem
				)
			}
		);
	}

	#[test]
	fn parses_inline_list_item() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "inline list-item", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 16),
				node: DisplayValue::PairAndMarker(
					DisplayOutside::Inline,
					DisplayInside::Flow,
					DisplayMarker::ListItem
				)
			}
		);
	}

	#[test]
	fn parses_inline_flow_root_list_item() {
		let allocator = Allocator::default();
		let parser =
			Parser::new(&allocator, "inline flow-root list-item", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 26),
				node: DisplayValue::PairAndMarker(
					DisplayOutside::Inline,
					DisplayInside::FlowRoot,
					DisplayMarker::ListItem
				)
			}
		);
	}

	#[test]
	fn parses_ruby_run_in() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "ruby run-in", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 11),
				node: DisplayValue::Pair(DisplayOutside::RunIn, DisplayInside::Ruby)
			}
		);
	}

	#[test]
	fn errors_block_inline() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "block inline", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		assert_eq!(parser_return.errors.len(), 1);
	}

	#[test]
	fn errors_flow_root_inline_list_item() {
		let allocator = Allocator::default();
		let parser =
			Parser::new(&allocator, "flow-root inline list-item", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<DisplayValue>();
		assert_eq!(parser_return.errors.len(), 1);
	}
}
