use hdx_atom::Atom;
use hdx_lexer::Token;

use crate::{parser::Parser, span::Spanned, Result, Vec, unexpected, expect, peek, discard};

use super::Parse;

// Parses various "Selector Lists" into their units
// https://drafts.csswg.org/selectors-4/#typedef-selector-list
// https://drafts.csswg.org/selectors-4/#typedef-relative-selector-list
// https://drafts.csswg.org/selectors-4/#typedef-complex-real-selector-list
// https://drafts.csswg.org/selectors-4/#typedef-relative-real-selector-list
pub trait SelectorList<'a>: Sized + Parse<'a> {
	// SelectorComponent represents a Selector, or Combinator.
	// https://drafts.csswg.org/selectors-4/#typedef-combinator
	// https://drafts.csswg.org/selectors-4/#typedef-type-selector
	// https://drafts.csswg.org/selectors-4/#typedef-subclass-selector
	// https://drafts.csswg.org/selectors-4/#typedef-pseudo-element-selector
	type SelectorComponent: SelectorComponent<'a>;

	fn parse_selector_list(
		parser: &mut Parser<'a>,
	) -> Result<Vec<'a, Spanned<Vec<'a, Self::SelectorComponent>>>> {
		discard!(parser, Token::Whitespace);
		let mut selectors = parser.new_vec();
		loop {
			let mut span = parser.span();
			let mut selector = parser.new_vec();
			'units: loop {
				let component = Self::SelectorComponent::parse_selector_component(selector.iter().last(), parser)?;
				selector.push(component);
				match parser.cur() {
					Token::Comma | Token::LeftCurly | Token::RightParen | Token::Eof => {
						span = span.end(parser.pos());
						break 'units;
					}
					Token::Whitespace if peek!(parser, Token::Comma | Token::LeftCurly | Token::RightParen | Token::Eof) => {
						parser.advance();
						span = span.end(parser.pos());
						break 'units;
					}
					_ => {}
				}
			}
			selectors.push(Spanned { node: selector, span: span.end(parser.pos()) });
			match parser.cur() {
				Token::Comma => {
					parser.advance();
				}
				_ => break,
			}
		}
		Ok(selectors)
	}
}

pub trait SelectorComponent<'a>: Sized {
	fn wildcard() -> Self;

	fn id_from_atom(atom: Atom) -> Option<Self>;
	fn class_from_atom(atom: Atom) -> Option<Self>;
	fn type_from_atom(atom: Atom) -> Option<Self>;
	fn pseudo_class_from_atom(atom: Atom) -> Option<Self>;
	fn legacy_pseudo_element_from_token(atom: Atom) -> Option<Self>;
	fn pseudo_element_from_atom(atom: Atom) -> Option<Self>;

	fn ns_type_from_token(ns_token: &Token, type_token: &Token) -> Option<Self>;

	fn parse_combinator(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_attribute(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_class(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_element(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_selector_component(prev: Option<&Self>, parser: &mut Parser<'a>) -> Result<Self> {
		let node = match parser.cur() {
			Token::Ident(atom) => {
				let ns_token = parser.cur();
				parser.advance_including_whitespace();
				match parser.cur() {
					Token::Delim('|') => {
						parser.advance_including_whitespace();
						expect!(parser, Token::Delim('*') | Token::Ident(_));
						let s = Self::ns_type_from_token(&ns_token, &parser.cur());
						if s.is_some() {
							parser.advance_including_whitespace();
						}
						s
					},
					_ => Self::type_from_atom(atom)
				}
			},
			Token::HashId(atom) => {
				let s = Self::type_from_atom(atom);
				if s.is_some() {
					parser.advance_including_whitespace();
				}
				s
			},
			Token::LeftSquare => {
				parser.advance();
				let attr = Self::parse_attribute(parser)?;
				expect!(parser, Token::RightSquare);
				parser.advance_including_whitespace();
				Some(attr)
			},
			Token::Delim(ch) => match ch {
				'.' if peek!(parser, Token::Ident(_)) => {
					parser.advance_including_whitespace();
					match parser.cur() {
						Token::Ident(atom) => {
							let s = Self::class_from_atom(atom);
							if s.is_some() {
								parser.advance_including_whitespace();
							}
							s
						}
						token => unexpected!(parser, token)
					}
				},
				'*' => {
					parser.advance_including_whitespace();
					match parser.cur() {
						Token::Delim('|') => {
							parser.advance_including_whitespace();
							match parser.cur() {
								token @ Token::Ident(_) => {
									let s = Self::ns_type_from_token(&Token::Delim('*'), &token);
									if s.is_some() {
										parser.advance_including_whitespace();
									}
									s
								}
								token => unexpected!(parser, token),
							}
						},
						_ => Some(Self::wildcard())
					}
				},
				_ => Some(Self::parse_combinator(parser)?),
			},
			Token::Colon => {
				parser.advance_including_whitespace();
				match parser.cur() {
					Token::Colon => {
						parser.advance_including_whitespace();
						match parser.cur() {
							Token::Ident(atom) => Self::pseudo_element_from_atom(atom),
							Token::Function(_) => Some(Self::parse_functional_pseudo_element(parser)?),
							token => unexpected!(parser, token),
						}
					},
					Token::Ident(atom) => {
						let pseudo = Self::legacy_pseudo_element_from_token(atom.clone())
							.or_else(|| Self::pseudo_class_from_atom(atom));
						if pseudo.is_some() {
							parser.advance_including_whitespace();
						}
						pseudo
					},
					Token::Function(_) => Some(Self::parse_functional_pseudo_class(parser)?),
					token => unexpected!(parser, token)
				}
			}
			_ => Some(Self::parse_combinator(parser)?)
		};
		if let Some(node) = node {
			Ok(node)
		} else {
			unexpected!(parser)
		}
	}

}
