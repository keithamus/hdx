use bumpalo::collections::Vec;
use css_lexer::{Kind, KindSet};

use crate::{diagnostics, Build, Parse, Parser, Peek, Result, T};

/// Parses a "Selector Lists" into a comma separated list of nodes that implement [CompoundSelector].
///
/// ```md
/// <selector-list>
///  │├─╭─ <compound-selector> ─╮─ "," ─╭─╮─┤│
///     │                       ╰───────╯ │
///     ╰─────────────────────────────────╯
/// ```
///
/// The various selectors lists can implement this trait, for example [Selector List][1], [Relative Selector List][2],
/// [Complex Real Selector List][3], [Relative Real Selector List][4].
///
/// [1]: https://drafts.csswg.org/selectors-4/#typedef-selector-list
/// [2]: https://drafts.csswg.org/selectors-4/#typedef-relative-selector-list
/// [3]: https://drafts.csswg.org/selectors-4/#typedef-complex-real-selector-list
/// [4]: https://drafts.csswg.org/selectors-4/#typedef-relative-real-selector-list
pub trait SelectorList<'a>: Sized + Parse<'a> {
	type CompoundSelector: Parse<'a> + CompoundSelector<'a>;

	fn parse_selector_list(p: &mut Parser<'a>) -> Result<Vec<'a, (Self::CompoundSelector, Option<T![,]>)>> {
		let mut selectors = Vec::new_in(p.bump());
		loop {
			if p.at_end() || p.peek_n(1) == KindSet::LEFT_CURLY_RIGHT_PAREN_OR_SEMICOLON {
				break;
			}
			let selector = p.parse::<Self::CompoundSelector>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			selectors.push((selector, comma));
		}
		Ok(selectors)
	}
}

pub trait CompoundSelector<'a>: Sized + Parse<'a> {
	// SelectorComponent represents a Selector, or Combinator.
	// https://drafts.csswg.org/selectors-4/#typedef-combinator
	// https://drafts.csswg.org/selectors-4/#typedef-type-selector
	// https://drafts.csswg.org/selectors-4/#typedef-subclass-selector
	// https://drafts.csswg.org/selectors-4/#typedef-pseudo-element-selector
	type SelectorComponent: Parse<'a> + SelectorComponent<'a>;

	fn parse_compound_selector(p: &mut Parser<'a>) -> Result<Vec<'a, Self::SelectorComponent>> {
		let mut components = Vec::new_in(p.bump());
		// Trim leading whitespace
		p.consume_trivia();
		loop {
			// If a stop token has been reached, break the loop
			if p.at_end() || p.peek_n(1) == KindSet::LEFT_CURLY_RIGHT_PAREN_COMMA_OR_SEMICOLON {
				break;
			}
			components.push(p.parse::<Self::SelectorComponent>()?);
		}
		Ok(components)
	}
}

pub trait SelectorComponent<'a>: Sized {
	type Wildcard: Peek<'a> + Build<'a>;
	type Id: Peek<'a> + Build<'a>;
	type Type: Peek<'a> + Build<'a>;
	type PseudoClass: Parse<'a>;
	type PseudoElement: Parse<'a>;
	type LegacyPseudoElement: Peek<'a> + Parse<'a>;
	type Class: Parse<'a>;
	type NsType: Parse<'a>;
	type Combinator: Parse<'a>;
	type Attribute: Parse<'a>;
	type FunctionalPseudoClass: Parse<'a>;
	type FunctionalPseudoElement: Parse<'a>;

	fn build_wildcard(node: Self::Wildcard) -> Self;
	fn build_id(node: Self::Id) -> Self;
	fn build_class(node: Self::Class) -> Self;
	fn build_type(node: Self::Type) -> Self;
	fn build_pseudo_class(node: Self::PseudoClass) -> Self;
	fn build_pseudo_element(node: Self::PseudoElement) -> Self;
	fn build_legacy_pseudo_element(node: Self::LegacyPseudoElement) -> Self;
	fn build_ns_type(node: Self::NsType) -> Self;
	fn build_combinator(node: Self::Combinator) -> Self;
	fn build_attribute(node: Self::Attribute) -> Self;
	fn build_functional_pseudo_class(node: Self::FunctionalPseudoClass) -> Self;
	fn build_functional_pseudo_element(node: Self::FunctionalPseudoElement) -> Self;

	fn parse_selector_component(p: &mut Parser<'a>) -> Result<Self> {
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.peek_n(1);
		let t = c.token();
		match t.kind() {
			Kind::Ident => match p.peek_n(2) {
				t if t == '|' => {
					p.set_skip(skip);
					p.parse::<Self::NsType>().map(Self::build_ns_type)
				}
				_ => {
					let c = p.next();
					p.set_skip(skip);
					if Self::Type::peek(p, c) {
						Ok(Self::build_type(Self::Type::build(p, c)))
					} else {
						Err(diagnostics::UnexpectedTag(p.parse_str_lower(c).to_owned(), c.into()))?
					}
				}
			},
			Kind::Hash if t.hash_is_id_like() => {
				let c = p.next();
				p.set_skip(skip);
				if Self::Id::peek(p, c) {
					Ok(Self::build_id(Self::Id::build(p, c)))
				} else {
					Err(diagnostics::UnexpectedId(p.parse_str_lower(c).to_owned(), c.into()))?
				}
			}
			Kind::LeftSquare => {
				p.set_skip(skip);
				p.parse::<Self::Attribute>().map(Self::build_attribute)
			}
			Kind::Delim => match t.char().unwrap() {
				'.' => {
					let c = p.peek_n(2);
					p.set_skip(skip);
					match c.token().kind() {
						Kind::Ident => p.parse::<Self::Class>().map(Self::build_class),
						k => Err(diagnostics::ExpectedIdent(k, c.into()))?,
					}
				}
				'*' => {
					let t = p.peek_n(2);
					p.set_skip(skip);
					if t == '|' {
						p.parse::<Self::NsType>().map(Self::build_ns_type)
					} else {
						let c = p.next();
						Ok(Self::build_wildcard(Self::Wildcard::build(p, c)))
					}
				}
				_ => {
					p.set_skip(skip);
					p.parse::<Self::Combinator>().map(Self::build_combinator)
				}
			},
			Kind::Colon => {
				let c2 = p.peek_n(2);
				match c2.token().kind() {
					Kind::Colon => {
						let c3 = p.peek_n(3);
						p.set_skip(skip);
						match c3.token().kind() {
							Kind::Ident => p.parse::<Self::PseudoElement>().map(Self::build_pseudo_element),
							Kind::Function => {
								p.parse::<Self::FunctionalPseudoElement>().map(Self::build_functional_pseudo_element)
							}
							_ => Err(diagnostics::Unexpected(c3.into(), c3.into()))?,
						}
					}
					Kind::Ident => {
						p.set_skip(skip);
						if Self::LegacyPseudoElement::peek(p, c) {
							p.parse::<Self::LegacyPseudoElement>().map(Self::build_legacy_pseudo_element)
						} else {
							p.parse::<Self::PseudoClass>().map(Self::build_pseudo_class)
						}
					}
					Kind::Function => {
						p.set_skip(skip);
						p.parse::<Self::FunctionalPseudoClass>().map(Self::build_functional_pseudo_class)
					}
					_ => Err(diagnostics::Unexpected(t.kind(), c2.into()))?,
				}
			}
			_ => {
				let value = p.parse::<Self::Combinator>().map(Self::build_combinator);
				// Given descendant combinators cannot appear in sequence with other combinators, we can safely eat trivia here
				// in order to remove unecessary conjoined descendant combinators
				p.set_skip(KindSet::WHITESPACE);
				p.consume_trivia();
				p.set_skip(skip);
				value
			}
		}
	}
}
