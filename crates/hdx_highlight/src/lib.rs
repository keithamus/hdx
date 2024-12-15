use bitmask_enum::bitmask;
use core::fmt;
use hdx_lexer::Span;
use std::collections::HashMap;

mod css;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;

use css::*;

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenTypes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticKind {
	Tag,
	Property,
	Punctuation,
}

impl fmt::Display for SemanticKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Tag => write!(f, "tag"),
			Self::Property => write!(f, "property"),
			Self::Punctuation => write!(f, "punctuation"),
		}
	}
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenModifiers
#[bitmask(u8)]
enum SemanticModifier {
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

	fn insert(&mut self, span: Span, kind: SemanticKind, modifier: SemanticModifier) {
		self.highlights.insert(span, Highlight { span, kind, modifier });
	}
}
