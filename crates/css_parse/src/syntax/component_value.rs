use super::prelude::*;
use crate::{AssociatedWhitespaceRules, Box, FunctionBlock, Result, SimpleBlock};

/// <https://drafts.csswg.org/css-syntax-3/#consume-component-value>
///
/// A compatible "Token" per CSS grammar, subsetted to the tokens possibly
/// rendered by ComponentValue (so no pairwise, function tokens, etc).
#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(Box<'a, SimpleBlock<'a>>),
	Function(Box<'a, FunctionBlock<'a>>),
	Whitespace(T![Whitespace]),
	Number(T![Number]),
	Dimension(T![Dimension]),
	Ident(T![Ident]),
	AtKeyword(T![AtKeyword]),
	Hash(T![Hash]),
	String(T![String]),
	Url(T![Url]),
	Delim(T![Delim]),
	Colon(T![:]),
	Semicolon(T![;]),
	Comma(T![,]),
}

impl<'a, M: NodeMetadata> NodeWithMetadata<M> for ComponentValue<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}

impl<'a> Peek<'a> for ComponentValue<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[
		Kind::Whitespace,
		Kind::Number,
		Kind::Dimension,
		Kind::Ident,
		Kind::AtKeyword,
		Kind::Hash,
		Kind::String,
		Kind::Url,
		Kind::Delim,
		Kind::Colon,
		Kind::Semicolon,
		Kind::Comma,
		Kind::Function,
		Kind::LeftCurly,
		Kind::LeftParen,
		Kind::LeftSquare,
	]);
	#[inline(always)]
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		c == Self::PEEK_KINDSET || <T![' ']>::peek(p, c)
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		Ok(if <T![' ']>::peek(p, c) {
			Self::Whitespace(p.parse::<T![' ']>()?)
		} else if <T![PairWiseStart]>::peek(p, c) {
			let old_state = p.set_state(State::Nested);
			let block = p.parse::<Box<SimpleBlock>>();
			p.set_state(old_state);
			Self::SimpleBlock(block?)
		} else if <T![Function]>::peek(p, c) {
			Self::Function(p.parse::<Box<FunctionBlock>>()?)
		} else if <T![Number]>::peek(p, c) {
			Self::Number(p.parse::<T![Number]>()?)
		} else if <T![Dimension]>::peek(p, c) {
			Self::Dimension(p.parse::<T![Dimension]>()?)
		} else if <T![Ident]>::peek(p, c) {
			Self::Ident(p.parse::<T![Ident]>()?)
		} else if <T![AtKeyword]>::peek(p, c) {
			Self::AtKeyword(p.parse::<T![AtKeyword]>()?)
		} else if <T![Hash]>::peek(p, c) {
			Self::Hash(p.parse::<T![Hash]>()?)
		} else if <T![String]>::peek(p, c) {
			Self::String(p.parse::<T![String]>()?)
		} else if <T![Url]>::peek(p, c) {
			Self::Url(p.parse::<T![Url]>()?)
		} else if <T![Delim]>::peek(p, c) {
			p.parse::<T![Delim]>().map(|delim| {
				// Carefully handle Whitespace rules to ensure whitespace isn't lost when re-serializing
				let mut rules = AssociatedWhitespaceRules::none();
				if p.peek_n_with_skip(1, KindSet::COMMENTS) == Kind::Whitespace {
					rules |= AssociatedWhitespaceRules::EnforceAfter;
				} else {
					rules |= AssociatedWhitespaceRules::BanAfter;
				}
				Self::Delim(delim.with_associated_whitespace(rules))
			})?
		} else if <T![:]>::peek(p, c) {
			Self::Colon(p.parse::<T![:]>()?)
		} else if <T![;]>::peek(p, c) {
			Self::Semicolon(p.parse::<T![;]>()?)
		} else if <T![,]>::peek(p, c) {
			Self::Comma(p.parse::<T![,]>()?)
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		})
	}
}

impl<'a> ToCursors for ComponentValue<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::SimpleBlock(t) => ToCursors::to_cursors(t, s),
			Self::Function(t) => ToCursors::to_cursors(t, s),
			Self::Ident(t) => ToCursors::to_cursors(t, s),
			Self::AtKeyword(t) => ToCursors::to_cursors(t, s),
			Self::Hash(t) => ToCursors::to_cursors(t, s),
			Self::String(t) => ToCursors::to_cursors(t, s),
			Self::Url(t) => ToCursors::to_cursors(t, s),
			Self::Delim(t) => ToCursors::to_cursors(t, s),
			Self::Number(t) => ToCursors::to_cursors(t, s),
			Self::Dimension(t) => ToCursors::to_cursors(t, s),
			Self::Whitespace(t) => ToCursors::to_cursors(t, s),
			Self::Colon(t) => ToCursors::to_cursors(t, s),
			Self::Semicolon(t) => ToCursors::to_cursors(t, s),
			Self::Comma(t) => ToCursors::to_cursors(t, s),
		}
	}
}

impl<'a> ToSpan for ComponentValue<'a> {
	fn to_span(&self) -> Span {
		match self {
			Self::SimpleBlock(t) => t.to_span(),
			Self::Function(t) => t.to_span(),
			Self::Ident(t) => t.to_span(),
			Self::AtKeyword(t) => t.to_span(),
			Self::Hash(t) => t.to_span(),
			Self::String(t) => t.to_span(),
			Self::Url(t) => t.to_span(),
			Self::Delim(t) => t.to_span(),
			Self::Number(t) => t.to_span(),
			Self::Dimension(t) => t.to_span(),
			Self::Whitespace(t) => t.to_span(),
			Self::Colon(t) => t.to_span(),
			Self::Semicolon(t) => t.to_span(),
			Self::Comma(t) => t.to_span(),
		}
	}
}

impl<'a> SemanticEq for ComponentValue<'a> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		match (self, other) {
			(Self::SimpleBlock(a), Self::SimpleBlock(b)) => a.semantic_eq(b, source_text),
			(Self::Function(a), Self::Function(b)) => a.semantic_eq(b, source_text),
			(Self::Number(a), Self::Number(b)) => a.semantic_eq(b, source_text),
			(Self::Dimension(a), Self::Dimension(b)) => a.semantic_eq(b, source_text),
			(Self::Ident(a), Self::Ident(b)) => a.semantic_eq(b, source_text),
			(Self::AtKeyword(a), Self::AtKeyword(b)) => a.semantic_eq(b, source_text),
			(Self::Hash(a), Self::Hash(b)) => a.semantic_eq(b, source_text),
			(Self::String(a), Self::String(b)) => a.semantic_eq(b, source_text),
			(Self::Url(a), Self::Url(b)) => a.semantic_eq(b, source_text),
			(Self::Delim(a), Self::Delim(b)) => a.semantic_eq(b, source_text),
			(Self::Colon(a), Self::Colon(b)) => a.semantic_eq(b, source_text),
			(Self::Semicolon(a), Self::Semicolon(b)) => a.semantic_eq(b, source_text),
			(Self::Comma(a), Self::Comma(b)) => a.semantic_eq(b, source_text),
			// Whitespace has no semantic relevance, other than its presence, so it should always be true
			(Self::Whitespace(_), Self::Whitespace(_)) => true,
			_ => false, // Different variants are never equal
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EmptyAtomSet, test_helpers::*};

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValue, "foo");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValue, " ");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValue, "{block}");
	}
}
