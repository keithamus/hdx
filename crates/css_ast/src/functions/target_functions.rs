use super::prelude::*;
use crate::{CssAtomSet, types::CounterStyle};
use css_parse::Box;

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TextFunctionContent {
	#[atom(CssAtomSet::Content)]
	Content(T![Ident]),
	#[atom(CssAtomSet::Before)]
	Before(T![Ident]),
	#[atom(CssAtomSet::After)]
	After(T![Ident]),
	#[atom(CssAtomSet::FirstLetter)]
	FirstLetter(T![Ident]),
}

#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TargetCounterKind {
	String(T![String]),
	Url(T![Url]),
}

/// <https://drafts.csswg.org/css-content-3/#typedef-target>
///
/// ```text,ignore
/// <target> = <target-counter()> | <target-counters()> | <target-text()>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Target<'a> {
	/// <https://drafts.csswg.org/css-content-3/#target-counter>
	///
	/// ```text,ignore
	/// target-counter() = target-counter( [ <string> | <url> ] , <custom-ident> , <counter-style>? )
	/// ```
	TargetCounter(Box<'a, TargetCounterFunction<'a>>),
	/// <https://drafts.csswg.org/css-content-3/#target-counters>
	///
	/// ```text,ignore
	/// target-counters() = target-counters( [ <string> | <url> ] , <custom-ident> , <string> , <counter-style>? )
	/// ```
	TargetCounters(Box<'a, TargetCountersFunction<'a>>),
	/// <https://drafts.csswg.org/css-content-3/#target-text>
	///
	/// ```text,ignore
	/// target-text() = target-text( [ <string> | <url> ] , [ content | before | after | first-letter ]? )
	/// ```
	TargetText(TargetTextFunction),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCounterFunction<'a> {
	#[atom(CssAtomSet::TargetCounter)]
	pub name: T![Function],
	pub params: TargetCounterParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCounterParams<'a>(
	pub TargetCounterKind,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub T![Ident],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<CounterStyle<'a>>,
);

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCountersFunction<'a> {
	#[atom(CssAtomSet::TargetCounters)]
	pub name: T![Function],
	pub params: TargetCountersParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCountersParams<'a>(
	pub TargetCounterKind,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub T![Ident],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub T![String],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<CounterStyle<'a>>,
);

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetTextFunction {
	#[atom(CssAtomSet::TargetText)]
	pub name: T![Function],
	pub params: TargetTextParams,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetTextParams(
	pub TargetCounterKind,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<TextFunctionContent>,
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Target, "target-counter('foo',bar,lower-roman)");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-counters('foo',bar,'baz',lower-roman)");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-text('foo')");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-text('foo',before)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counter()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counter('foo')");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters('foo')");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters('foo',bar)");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-text()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-text(123)");
	}
}
