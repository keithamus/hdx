use super::prelude::*;
use crate::{CalcableValue, units::LengthPercentage};

/// <https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block>
///
/// ```text,ignore
/// snap-block() = snap-block( <length> , [ start | end | near ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SnapBlockFunction<'a> {
	#[atom(CssAtomSet::SnapBlock)]
	pub name: T![Function],
	pub params: SnapBlockFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SnapBlockFunctionParams<'a>(
	pub CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<SnapBlockKeyword>,
	#[semantic_eq(skip)] pub Option<T![,]>,
);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SnapBlockKeyword {
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
	#[atom(CssAtomSet::Near)]
	Near(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(10%)");
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(10%,start)");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(calc(1px + 2%))");
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(var(--x))");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-inline(10%)");
		assert_parse_error!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(start)");
	}
}
