use hdx_ast::css::selector::{
	Attribute, AttributeMatch, AttributeModifier, Combinator, Component, LegacyPseudoElement,
	NSPrefix, PseudoClass, PseudoElement, Selector,
};
use miette::Result;

use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Span, Spanned, Vec};

fn parse_wq_name(parser: &mut Parser) -> Result<(NSPrefix, Atom)> {
	let peeked = parser.peek();
	let mut nsprefix = NSPrefix::None;
	if peeked.kind == Kind::Delim && peeked.value.as_char().unwrap() == '|' {
		match parser.cur() {
			Kind::Delim => {
				let span = parser.span();
				let ch = parser.expect_delim()?;
				if ch == '*' {
					nsprefix = NSPrefix::Wildcard;
				} else {
					Err(diagnostics::UnexpectedDelim(ch, span))?
				}
			}
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				nsprefix = NSPrefix::Named(ident);
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
		let span = parser.span();
		let ch = parser.expect_delim()?;
		if ch != '|' {
			Err(diagnostics::UnexpectedDelim(ch, span))?
		}
		Ok((nsprefix, parser.expect_ident()?))
	} else {
		if parser.at(Kind::Delim) && parser.cur_char().unwrap() == '|' {
			parser.advance();
		}
		Ok((nsprefix, parser.expect_ident()?))
	}
}

impl<'a> Parse<'a> for Selector<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut components: Vec<'a, Spanned<Component>> = parser.new_vec();
		loop {
			match parser.cur() {
				Kind::Eof | Kind::Semicolon | Kind::Comma | Kind::LeftCurly => {
					break;
				}
				Kind::Whitespace => {
					if matches!(
						parser.peek().kind,
						Kind::Eof | Kind::Semicolon | Kind::Comma | Kind::LeftCurly
					) {
						break;
					}
				}
				_ => {}
			}
			let component = Component::parse(parser)?;
			if let Some(Spanned { node, span: component_span }) = components.last() {
				match (node, &component.node) {
					// A selector like `a /**/ b` would parse as // <Type>, <Descendant>,
					// <Descendant>, <Type>. The CSS selector grammar implicitly swallows adjacent
					// descendant combinators as whitespace, but due to simplifying AST nodes in our
					// parser, it means we must explicitly check for, and elide adjacent descendant
					// combinators. Adjacent Descendant Combinator Elision is the name of my metal
					// band, btw.
					(Component::Combinator(_), Component::Combinator(Combinator::Descendant))
					| (Component::Combinator(Combinator::Descendant), Component::Combinator(_)) => {
						continue;
					}
					// Combinators cannot be next to eachother.
					(Component::Combinator(_), Component::Combinator(_)) => {
						Err(diagnostics::AdjacentSelectorCombinators(
							*component_span,
							Span::new(span.start, component_span.start),
						))?
					}
					// Types cannot be next to eachother.
					(Component::Type(_), Component::Type(_)) => {
						Err(diagnostics::AdjacentSelectorTypes(
							*component_span,
							Span::new(span.start, component_span.start),
						))?
					}
					_ => {}
				}
			}
			components.push(component);
		}
		Ok(Self { components: parser.boxup(components) }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for Component<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Whitespace => {
				parser.advance();
				Ok(Self::Combinator(Combinator::Descendant).spanned(span.end(parser.pos())))
			}
			Kind::Ident => {
				let name = parser.cur().as_atom_lower().unwrap();
				parser.advance();
				Ok(Self::Type(name).spanned(span))
			}
			Kind::Colon => {
				parser.advance();
				match parser.cur() {
					Kind::Colon => {
						parser.advance();
						parser.expect_without_advance(Kind::Ident)?;
						let ident = parser.cur().as_atom().unwrap();
						if let Some(selector) = PseudoElement::from_atom(ident) {
							parser.advance();
							Ok(Self::PseudoElement(selector).spanned(span.end(parser.pos())))
						} else {
							Err(diagnostics::Unimplemented(parser.span()))?
						}
					}
					Kind::Ident => {
						parser.expect_without_advance(Kind::Ident)?;
						let ident = parser.cur().as_atom().unwrap();
						if let Some(selector) = PseudoClass::from_atom(ident.clone()) {
							parser.advance();
							Ok(Self::PseudoClass(selector).spanned(span.end(parser.pos())))
						} else if let Some(e) = LegacyPseudoElement::from_atom(ident.clone()) {
							parser.advance();
							Ok(Self::LegacyPseudoElement(e).spanned(span.end(parser.pos())))
						} else {
							Err(diagnostics::UnexpectedIdent(ident, parser.span()))?
						}
					}
					_ => Err(diagnostics::Unimplemented(parser.span()))?,
				}
			}
			Kind::Hash => {
				let name = parser.cur().as_atom().unwrap();
				parser.advance();
				Ok(Self::Id(name).spanned(span.end(parser.pos())))
			}
			Kind::Delim => match parser.cur().value.as_char() {
				Some('.') => {
					let next_token = parser.peek_including_trivia();
					match next_token.kind {
						Kind::Ident => {
							parser.advance();
							let ident = parser.cur().as_atom().unwrap();
							parser.advance();
							Ok(Self::Class(ident).spanned(span.end(parser.pos())))
						}
						_ => Err(diagnostics::Unimplemented(parser.span()))?,
					}
				}
				Some('*') => {
					let next_token = parser.peek_including_trivia();
					match next_token.kind {
						Kind::Delim if next_token.value.as_char().unwrap() == '|' => {
							let (prefix, atom) = parse_wq_name(parser)?;
							Ok(Self::NSPrefixedType(parser.boxup((prefix, atom)))
								.spanned(span.end(parser.pos())))
						}
						_ => {
							parser.advance();
							Ok(Self::Wildcard.spanned(span.end(parser.pos())))
						}
					}
				}
				_ => Err(diagnostics::Unimplemented(parser.span()))?,
			},
			Kind::LeftSquare => {
				let attr = Attribute::parse(parser)?;
				Ok(Component::Attribute(parser.boxup(attr)).spanned(span.end(parser.pos())))
			}
			_ => Err(diagnostics::Unimplemented(parser.span()))?,
		}
	}
}

impl<'a> Parse<'a> for Attribute {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		parser.expect(Kind::LeftSquare)?;
		let (ns_prefix, name) = parse_wq_name(parser)?;
		let mut matcher = AttributeMatch::Any;
		let mut modifier = AttributeModifier::None;
		let mut value = atom!("");
		match parser.cur() {
			Kind::RightSquare => {
				parser.advance();
				return Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.end(parser.pos())));
			}
			Kind::Delim => {
				let delim_span = parser.span();
				let ch = parser.cur().value.as_char().unwrap();
				parser.advance();
				if matcher != AttributeMatch::Any {
					Err(diagnostics::UnexpectedDelim(ch, delim_span))?;
				}
				matcher = match ch {
					'=' => AttributeMatch::Exact,
					'~' => AttributeMatch::SpaceList,
					'|' => AttributeMatch::LangPrefix,
					'^' => AttributeMatch::Prefix,
					'$' => AttributeMatch::Suffix,
					'*' => AttributeMatch::Contains,
					_ => Err(diagnostics::UnexpectedDelim(ch, delim_span))?,
				};
				if ch != '=' {
					let ch = parser.expect_delim()?;
					if ch != '=' {
						Err(diagnostics::UnexpectedDelim(ch, delim_span))?;
					}
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
		match parser.cur() {
			Kind::Ident | Kind::String => {
				value = parser.cur().as_atom().unwrap();
				parser.advance();
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
		match parser.cur() {
			Kind::RightSquare => {
				parser.advance();
				Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.end(parser.pos())))
			}
			Kind::Ident => {
				let ident_span = parser.span();
				modifier = match parser.expect_ident()? {
					atom!("i") => AttributeModifier::Insensitive,
					atom!("s") => AttributeModifier::Sensitive,
					a => Err(diagnostics::UnexpectedIdent(a, ident_span))?,
				};
				parser.expect(Kind::RightSquare)?;
				Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.end(parser.pos())))
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::css::selector::{
		Attribute, AttributeMatch, AttributeModifier, Combinator, Component, NSPrefix, Selector,
	};
	use oxc_allocator::{Box, Vec};

	use crate::{atom, Allocator, Atom, Parser, ParserOptions, Span, Spanned};

	#[test]
	fn test_descendant_combinator() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "a b  c   .d\n#f", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(0, 1), node: Component::Type(Atom::from("a")) });
		components.push(Spanned {
			span: Span::new(1, 2),
			node: Component::Combinator(Combinator::Descendant),
		});
		components.push(Spanned { span: Span::new(2, 3), node: Component::Type(Atom::from("b")) });
		components.push(Spanned {
			span: Span::new(3, 5),
			node: Component::Combinator(Combinator::Descendant),
		});
		components.push(Spanned { span: Span::new(5, 6), node: Component::Type(Atom::from("c")) });
		components.push(Spanned {
			span: Span::new(6, 9),
			node: Component::Combinator(Combinator::Descendant),
		});
		components
			.push(Spanned { span: Span::new(9, 11), node: Component::Class(Atom::from("d")) });
		components.push(Spanned {
			span: Span::new(11, 12),
			node: Component::Combinator(Combinator::Descendant),
		});
		components.push(Spanned { span: Span::new(12, 14), node: Component::Id(Atom::from("f")) });
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 14),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}

	#[test]
	fn test_class_auto() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, ".auto", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(0, 5), node: Component::Class(atom!("auto")) });
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 5),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}

	#[test]
	fn test_body_leading_trailing_space() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "   body   ", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(3, 7), node: Component::Type(atom!("body")) });
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(3, 7),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}

	#[test]
	fn test_attribute_selector() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "a[b='c']", ParserOptions::default());
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(0, 1), node: Component::Type(atom!("a")) });
		components.push(Spanned {
			span: Span::new(1, 8),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(1, 8),
				node: Attribute {
					ns_prefix: NSPrefix::None,
					name: atom!("b"),
					matcher: AttributeMatch::Exact,
					value: Atom::from("c"),
					modifier: AttributeModifier::None,
				},
			}))),
		});
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 8),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}

	#[test]
	fn test_attribute_with_ns_prefix() {
		let allocator = Allocator::default();
		let parser = Parser::new(
			&allocator,
			"a/**/[    */**/|/**/b    ~/*=*/= c/* */  /**/ s]",
			ParserOptions::default(),
		);
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(0, 1), node: Component::Type(atom!("a")) });
		components.push(Spanned {
			span: Span::new(5, 48),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(5, 48),
				node: Attribute {
					ns_prefix: NSPrefix::Wildcard,
					name: atom!("b"),
					value: Atom::from("c"),
					matcher: AttributeMatch::SpaceList,
					modifier: AttributeModifier::Sensitive,
				},
			}))),
		});
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 48),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}

	#[test]
	fn test_various_attribute_patterns() {
		let allocator = Allocator::default();
		let parser = Parser::new(
			&allocator,
			"a[b|a][b][*|b~=i i][|i|=i i][s^=s s][i|i$=\"i\"][*|i*='foo' s]",
			ParserOptions::default(),
		);
		let parser_return = parser.parse_entirely_with::<Selector>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		let ast = parser_return.output.unwrap();
		let mut components = Vec::new_in(&allocator);
		components.push(Spanned { span: Span::new(0, 1), node: Component::Type(atom!("a")) });
		// [b|a]
		components.push(Spanned {
			span: Span::new(1, 6),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(1, 6),
				node: Attribute {
					ns_prefix: NSPrefix::Named(atom!("b")),
					name: atom!("a"),
					matcher: AttributeMatch::Any,
					value: atom!(""),
					modifier: AttributeModifier::None,
				},
			}))),
		});
		// [b]
		components.push(Spanned {
			span: Span::new(6, 9),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(6, 9),
				node: Attribute {
					ns_prefix: NSPrefix::None,
					name: atom!("b"),
					matcher: AttributeMatch::Any,
					value: atom!(""),
					modifier: AttributeModifier::None,
				},
			}))),
		});
		// [*|b~=i i]
		components.push(Spanned {
			span: Span::new(9, 19),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(9, 19),
				node: Attribute {
					ns_prefix: NSPrefix::Wildcard,
					name: atom!("b"),
					matcher: AttributeMatch::SpaceList,
					value: atom!("i"),
					modifier: AttributeModifier::Insensitive,
				},
			}))),
		});
		// [|i|=i i]
		components.push(Spanned {
			span: Span::new(19, 28),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(19, 28),
				node: Attribute {
					ns_prefix: NSPrefix::None,
					name: atom!("i"),
					matcher: AttributeMatch::LangPrefix,
					value: atom!("i"),
					modifier: AttributeModifier::Insensitive,
				},
			}))),
		});
		// [s^=s s]
		components.push(Spanned {
			span: Span::new(28, 36),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(28, 36),
				node: Attribute {
					ns_prefix: NSPrefix::None,
					name: atom!("s"),
					matcher: AttributeMatch::Prefix,
					value: atom!("s"),
					modifier: AttributeModifier::Sensitive,
				},
			}))),
		});
		// [i|i$="i"]
		components.push(Spanned {
			span: Span::new(36, 46),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(36, 46),
				node: Attribute {
					ns_prefix: NSPrefix::Named(atom!("i")),
					name: atom!("i"),
					matcher: AttributeMatch::Suffix,
					value: atom!("i"),
					modifier: AttributeModifier::None,
				},
			}))),
		});
		// [*|i*='foo' s]
		components.push(Spanned {
			span: Span::new(46, 60),
			node: Component::Attribute(Box(allocator.alloc(Spanned {
				span: Span::new(46, 60),
				node: Attribute {
					ns_prefix: NSPrefix::Wildcard,
					name: atom!("i"),
					matcher: AttributeMatch::Contains,
					value: Atom::from("foo"),
					modifier: AttributeModifier::Sensitive,
				},
			}))),
		});
		assert_eq!(
			ast,
			Spanned {
				span: Span::new(0, 60),
				node: Selector { components: Box(allocator.alloc(components)) }
			}
		);
	}
}
