use super::prelude::*;

/// <https://drafts.csswg.org/animation-triggers-1/#funcdef-event-function-keypress>
///
/// ```text
/// keypress( <string>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct KeypressFunction {
	#[atom(CssAtomSet::Keypress)]
	pub name: T![Function],
	pub params: Option<T![String]>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, KeypressFunction, "keypress('a')");
		assert_parse!(CssAtomSet::ATOMS, KeypressFunction, "keypress()");
	}
}
