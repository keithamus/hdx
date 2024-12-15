use bitmask_enum::bitmask;
use core::fmt;
use hdx_lexer::Span;
use std::collections::HashMap;
use strum::{Display, VariantNames};

mod css;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;

use css::*;

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenTypes
#[derive(Display, VariantNames, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticKind {
	/* Selector Elements */
	Id,
	Tag,
	Class,
	Wildcard,
	Attribute,
	Namespace,
	Combinator,
	PseudoClass,
	PseudoElement,
	LegacyPseudoElement,
	FunctionalPseudoClass,
	FunctionalPseudoElement,

	/* Rule Elements */
	AtKeyword,
	Prelude,

	/* Property Declarations */
	Declaration,
	StyleValueKeyword,
	StyleValueDimension,
	StyleValueNumber,

	Punctuation,
}

impl SemanticKind {
	pub fn bits(&self) -> u8 {
		*self as u8
	}
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenModifiers
#[derive(VariantNames)]
#[bitmask(u8)]
pub enum SemanticModifier {
	Unknown,
	Deprecated,
	Experimental,
	Custom,
}

impl fmt::Display for SemanticModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(Self::Unknown) {
			write!(f, " unknown")?;
		}
		if self.contains(Self::Deprecated) {
			write!(f, " deprecated")?;
		}
		if self.contains(Self::Experimental) {
			write!(f, " experimental")?;
		}
		if self.contains(Self::Custom) {
			write!(f, " custom")?;
		}
		Ok(())
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Highlight {
	kind: SemanticKind,
	modifier: SemanticModifier,
	span: Span,
}

impl Highlight {
	#[inline(always)]
	pub fn span(&self) -> Span {
		self.span
	}

	#[inline(always)]
	pub fn modifier(&self) -> SemanticModifier {
		self.modifier
	}

	#[inline(always)]
	pub fn kind(&self) -> SemanticKind {
		self.kind
	}
}

#[derive(Default)]
pub struct TokenHighlighter {
	highlights: HashMap<Span, Highlight>,
}

impl TokenHighlighter {
	pub fn new() -> Self {
		Self { highlights: HashMap::new() }
	}

	pub fn get(&self, span: Span) -> Option<&Highlight> {
		self.highlights.get(&span)
	}

	pub fn highlights(&self) -> impl Iterator<Item = &Highlight> {
		self.highlights.values()
	}

	fn insert(&mut self, span: Span, kind: SemanticKind, modifier: SemanticModifier) {
		self.highlights.insert(span, Highlight { span, kind, modifier });
	}
}
