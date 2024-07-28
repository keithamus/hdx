use hdx_atom::Atom;
use hdx_lexer::{Include, Kind};

use crate::{diagnostics, discard, parser::Parser, peek, span::Spanned, unexpected, Result, Vec};

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
			while discard!(parser, Include::Whitespace, Kind::Whitespace) {}
			let span = parser.span();
			let mut selector = parser.new_vec();
			while !peek!(parser, Kind::Comma | Kind::LeftCurly | Kind::RightParen | Kind::Eof) {
				if peek!(parser, Kind::Whitespace)
					&& peek!(parser, 2, Kind::Comma | Kind::LeftCurly | Kind::RightParen | Kind::Eof)
				{
					parser.next_with(Include::Whitespace);
				} else {
					selector.push(Self::SelectorComponent::parse(parser)?);
				}
			}
			selectors.push(Spanned { node: selector, span: span.end(parser.pos()) });
			if !discard!(parser, Kind::Comma) {
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

	fn ns_type_from_token(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_combinator(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_attribute(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_class(parser: &mut Parser<'a>) -> Result<Self>;
	fn parse_functional_pseudo_element(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_selector_component(parser: &mut Parser<'a>) -> Result<Self> {
		let peek = parser.peek_with(Include::Whitespace);
		match peek.kind() {
			Kind::Ident => match parser.peek_n_with(2, Include::Whitespace) {
				t if t.kind() == Kind::Delim && matches!(t.char(), Some('|')) => {
					parser.next_with(Include::Whitespace);
					Self::ns_type_from_token(parser)
				}
				_ => {
					parser.next();
					let atom = parser.parse_atom(peek);
					Self::type_from_atom(&atom).ok_or_else(|| diagnostics::UnexpectedTag(atom, parser.span()).into())
				}
			},
			Kind::Hash if peek.hash_is_id_like() => {
				parser.next();
				let atom = parser.parse_atom(peek);
				Self::type_from_atom(&atom).ok_or_else(|| diagnostics::UnexpectedId(atom.clone(), parser.span()).into())
			}
			Kind::LeftSquare => Ok(Self::parse_attribute(parser)?),
			Kind::Delim => match peek.char().unwrap() {
				'.' => {
					parser.next();
					match parser.next_with(Include::Whitespace) {
						t if t.kind() == Kind::Ident => {
							let atom = parser.parse_atom(t);
							Self::class_from_atom(&atom)
								.ok_or_else(|| diagnostics::UnexpectedIdent(atom, parser.span()).into())
						}
						token => unexpected!(parser, token),
					}
				}
				'*' => match parser.peek_n_with(2, Include::Whitespace) {
					t if t.kind() == Kind::Delim && matches!(t.char(), Some('|')) => Self::ns_type_from_token(parser),
					_ => {
						parser.next();
						Ok(Self::wildcard())
					}
				},
				_ => Self::parse_combinator(parser),
			},
			Kind::Colon => {
				parser.next();
				let peek = parser.peek_with(Include::Whitespace);
				match peek.kind() {
					Kind::Colon => {
						parser.next_with(Include::Whitespace);
						let next = parser.next_with(Include::Whitespace);
						match next.kind() {
							Kind::Ident => {
								let atom = parser.parse_atom(next);
								Self::pseudo_element_from_atom(&atom).ok_or_else(|| {
									diagnostics::UnexpectedPseudoElement(atom.clone(), parser.span()).into()
								})
							}
							Kind::Function => Self::parse_functional_pseudo_element(parser),
							_ => unexpected!(parser, next),
						}
					}
					Kind::Ident => {
						let atom = parser.parse_atom(peek);
						parser.next_with(Include::Whitespace);
						Self::legacy_pseudo_element_from_token(&atom)
							.or_else(|| Self::pseudo_class_from_atom(&atom))
							.ok_or_else(|| diagnostics::UnexpectedPseudoClass(atom.clone(), parser.span()).into())
					}
					Kind::Function => Self::parse_functional_pseudo_class(parser),
					_ => unexpected!(parser, peek),
				}
			}
			_ => Self::parse_combinator(parser),
		}
	}
}
