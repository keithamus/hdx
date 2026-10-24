use super::prelude::*;
use crate::{CalcableValue, units::LengthPercentage};

/// <https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline>
///
/// ```text,ignore
/// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SnapInlineFunction<'a> {
	#[atom(CssAtomSet::SnapInline)]
	pub name: T![Function],
	pub params: SnapInlineFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SnapInlineFunctionParams<'a>(
	pub CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<SnapInlineKeyword>,
	#[semantic_eq(skip)] pub Option<T![,]>,
);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SnapInlineKeyword {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
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
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(10%)");
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(10%,near)");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(calc(1px + 2%))");
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(var(--x))");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-block(10%)");
		assert_parse_error!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(near)");
	}
}
