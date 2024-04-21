use hdx_atom::Atom;
use hdx_lexer::{Include, Token};

use crate::{diagnostics, discard, expect, parser::Parser, peek, span::Spanned, unexpected, Result, Vec};

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
	type SelectorComponent: Parse<'a>;

	fn parse_selector_list(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Vec<'a, Self::SelectorComponent>>>> {
		let mut selectors = parser.new_vec();
		loop {
			discard!(parser, Include::Whitespace, Token::Whitespace);
			let span = parser.span();
			let mut selector = parser.new_vec();
			while !peek!(parser, Token::Comma | Token::LeftCurly | Token::RightParen | Token::Eof) {
				if peek!(parser, Token::Whitespace)
					&& peek!(parser, 2, Token::Comma | Token::LeftCurly | Token::RightParen | Token::Eof)
				{
					parser.advance_with(Include::Whitespace);
				} else {
					selector.push(Self::SelectorComponent::parse(parser)?);
				}
			}
			selectors.push(Spanned { node: selector, span: span.end(parser.pos()) });
			if !discard!(parser, Token::Comma) {
				break;
			}
		}
		Ok(selectors)
	}
}

pub trait SelectorComponent<'a>: Sized {
	fn wildcard() -> Self;

	fn id_from_atom(atom: &Atom) -> Option<Self>;
	fn class_from_atom(atom: &Atom) -> Option<Self>;
	fn type_from_atom(atom: &Atom) -> Option<Self>;
	fn pseudo_class_from_atom(atom: &Atom) -> Option<Self>;
	fn legacy_pseudo_element_from_token(atom: &Atom) -> Option<Self>;
	fn pseudo_element_from_atom(atom: &Atom) -> Option<Self>;

	fn ns_type_from_token(ns_token: &Token, type_token: &Token) -> Option<Self>;

	fn parse_combinator(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_attribute(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_class(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_element(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_selector_component(parser: &mut Parser<'a>) -> Result<Self> {
		match parser.peek_with(Include::Whitespace).clone() {
			Token::Ident(ref atom) => {
				parser.advance();
				match parser.peek_with(Include::Whitespace) {
					Token::Delim('|') => {
						let ns_token = parser.cur().clone();
						parser.advance_with(Include::Whitespace);
						expect!(parser.next(), Token::Delim('*') | Token::Ident(_));
						Self::ns_type_from_token(&ns_token, parser.cur())
							.ok_or_else(|| diagnostics::UnexpectedIdent(atom.clone(), parser.span()).into())
					}
					_ => Self::type_from_atom(atom)
						.ok_or_else(|| diagnostics::UnexpectedTag(atom.clone(), parser.span()).into()),
				}
			}
			Token::HashId(ref atom) => {
				parser.advance();
				Self::type_from_atom(atom).ok_or_else(|| diagnostics::UnexpectedId(atom.clone(), parser.span()).into())
			}
			Token::LeftSquare => Ok(Self::parse_attribute(parser)?),
			Token::Delim(ch) => match ch {
				'.' => {
					parser.advance();
					match parser.next_with(Include::Whitespace).clone() {
						Token::Ident(atom) => Self::class_from_atom(&atom)
							.ok_or_else(|| diagnostics::UnexpectedIdent(atom.clone(), parser.span()).into()),
						token => unexpected!(parser, token),
					}
				}
				'*' => {
					parser.advance_with(Include::Whitespace);
					match parser.next_with(Include::Whitespace) {
						Token::Delim('|') => match parser.next_with(Include::Whitespace).clone() {
							token @ Token::Ident(_) => {
								let val = Self::ns_type_from_token(&Token::Delim('*'), &token).ok_or_else(|| {
									if let Token::Ident(atom) = token {
										diagnostics::UnexpectedTag(atom.clone(), parser.span()).into()
									} else {
										unreachable!()
									}
								});
								parser.peek_with(Include::Whitespace);
								val
							}
							token => unexpected!(parser, token),
						},
						_ => Ok(Self::wildcard()),
					}
				}
				_ => Self::parse_combinator(parser),
			},
			Token::Colon => {
				parser.advance();
				match parser.peek_with(Include::Whitespace).clone() {
					Token::Colon => {
						parser.advance_with(Include::Whitespace);
						match parser.next_with(Include::Whitespace).clone() {
							Token::Ident(atom) => Self::pseudo_element_from_atom(&atom).ok_or_else(|| {
								diagnostics::UnexpectedPseudoElement(atom.clone(), parser.span()).into()
							}),
							Token::Function(_) => Self::parse_functional_pseudo_element(parser),
							token => unexpected!(parser, token),
						}
					}
					Token::Ident(ref atom) => {
						parser.advance_with(Include::Whitespace);
						Self::legacy_pseudo_element_from_token(atom)
							.or_else(|| Self::pseudo_class_from_atom(atom))
							.ok_or_else(|| diagnostics::UnexpectedPseudoClass(atom.clone(), parser.span()).into())
					}
					Token::Function(_) => Self::parse_functional_pseudo_class(parser),
					token => unexpected!(parser, token),
				}
			}
			_ => Self::parse_combinator(parser),
		}
	}
}
