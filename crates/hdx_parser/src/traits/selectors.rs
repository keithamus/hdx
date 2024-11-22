use hdx_atom::Atom;
use hdx_lexer::{Include, Kind, Span, Spanned};

use crate::{diagnostics, discard, parser::Parser, Result, Vec, T};

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

	fn parse_compound_selector(parser: &mut Parser<'a>) -> Result<Spanned<Vec<'a, Self::SelectorComponent>>> {
		let start = parser.offset();
		let mut selector = parser.new_vec();
		loop {
			let peeked_kind = parser.peek_n_with(1, Include::Whitespace).kind();
			// If a stop token has been reached, break the loop
			if parser.at_end() || matches!(peeked_kind, Kind::LeftCurly | Kind::RightParen | Kind::Comma) {
				break;
			}
			// Handle whitespace carefully; it could be a descendant combinator or just whitespace next to a stop token
			if peeked_kind == Kind::Whitespace
				&& matches!(parser.peek_n(1).kind(), Kind::LeftCurly | Kind::RightParen | Kind::Comma)
			{
				break;
			}
			selector.push(Self::SelectorComponent::parse(parser)?);
		}
		Ok(Spanned { node: selector, span: Span::new(start, parser.offset()) })
	}

	fn parse_selector_list(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Vec<'a, Self::SelectorComponent>>>> {
		let mut selectors = parser.new_vec();
		loop {
			if parser.at_end() {
				break;
			}
			// Discard all leading whitespace
			while discard!(parser, Include::Whitespace, Whitespace) {}
			let next_token_kind = parser.peek::<T![Any]>().map(|t| t.kind()).unwrap_or(Kind::Eof);
			if matches!(next_token_kind, Kind::LeftCurly | Kind::RightParen) {
				break;
			}
			selectors.push(Self::parse_compound_selector(parser)?);
			if !discard!(parser, Comma) {
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
		let token = parser.peek_n_with(1, Include::Whitespace);
		match token.kind() {
			Kind::Ident => match parser.peek_n_with(2, Include::Whitespace) {
				t if t.kind() == Kind::Delim && matches!(t.char(), Some('|')) => {
					parser.next_with(Include::Whitespace);
					Self::ns_type_from_token(parser)
				}
				_ => {
					parser.next();
					let atom = parser.parse_atom(token);
					Self::type_from_atom(&atom).ok_or_else(|| diagnostics::UnexpectedTag(atom, token.span()).into())
				}
			},
			Kind::Hash if token.hash_is_id_like() => {
				parser.next();
				let atom = parser.parse_atom(token);
				Self::type_from_atom(&atom).ok_or_else(|| diagnostics::UnexpectedId(atom.clone(), token.span()).into())
			}
			Kind::LeftSquare => Ok(Self::parse_attribute(parser)?),
			Kind::Delim => match token.char().unwrap() {
				'.' => {
					parser.next();
					match parser.next_with(Include::Whitespace) {
						t if t.kind() == Kind::Ident => {
							let atom = parser.parse_atom(t);
							Self::class_from_atom(&atom)
								.ok_or_else(|| diagnostics::UnexpectedIdent(atom, token.span()).into())
						}
						token => Err(diagnostics::ExpectedIdent(token, token.span()))?,
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
				let token = parser.peek_with::<T![Any]>(Include::Whitespace).unwrap();
				match token.kind() {
					Kind::Colon => {
						parser.next_with(Include::Whitespace);
						let next = parser.next_with(Include::Whitespace);
						match next.kind() {
							Kind::Ident => {
								let atom = parser.parse_atom(next);
								Self::pseudo_element_from_atom(&atom).ok_or_else(|| {
									diagnostics::UnexpectedPseudoElement(atom.clone(), token.span()).into()
								})
							}
							Kind::Function => Self::parse_functional_pseudo_element(parser),
							_ => Err(diagnostics::Unexpected(next, next.span()))?,
						}
					}
					Kind::Ident => {
						let atom = parser.parse_atom(token);
						parser.next_with(Include::Whitespace);
						Self::legacy_pseudo_element_from_token(&atom)
							.or_else(|| Self::pseudo_class_from_atom(&atom))
							.ok_or_else(|| diagnostics::UnexpectedPseudoClass(atom.clone(), token.span()).into())
					}
					Kind::Function => Self::parse_functional_pseudo_class(parser),
					_ => Err(diagnostics::Unexpected(token, token.span()))?,
				}
			}
			_ => Self::parse_combinator(parser),
		}
	}
}
