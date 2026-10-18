use css_parse::T;
use csskit_derives::*;

/// A reference to a custom media query, e.g. `(--modern)`.
///
/// <https://drafts.csswg.org/mediaqueries-5/#custom-mq>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CustomMediaFeature(T!['('], T![DashedIdent], #[semantic_eq(skip)] T![')']);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CustomMediaFeature, "(--modern)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CustomMediaFeature, "(modern)");
		assert_parse_error!(CssAtomSet::ATOMS, CustomMediaFeature, "(--modern:1)");
	}
}
