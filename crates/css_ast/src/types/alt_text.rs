use super::prelude::*;
use crate::{AttrFunction, Counter};
use css_parse::Box;

/// <https://drafts.csswg.org/css-content-3/#typedef-alt-text>
///
/// ```text,ignore
/// <alt-text> = [ <string> | <counter> | <attr()> ]+
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AltText<'a>(pub Vec<'a, ContentAltItem<'a>>);

/// One item of an [`AltText`].
///
/// ```text,ignore
/// <string> | <counter> | <attr()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContentAltItem<'a> {
	String(T![String]),
	Counter(Counter<'a>),
	AttrFunction(Box<'a, AttrFunction<'a>>),
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "\"hello\"");
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "counter(section)");
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, AltText, "\"Chapter \" counter(chapter) attr(title)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ContentAltItem, "open-quote");
	}
}
