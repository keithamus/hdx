use super::prelude::*;
use crate::types::CounterStyle;
use css_parse::Box;

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counter()>  =  counter( <counter-name>, <counter-style>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CounterFunction<'a> {
	#[atom(CssAtomSet::Counter)]
	pub name: T![Function],
	pub params: CounterFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CounterFunctionParams<'a>(
	pub T![Ident],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<CounterStyle<'a>>,
);

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counters()> = counters( <counter-name>, <string>, <counter-style>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CountersFunction<'a> {
	#[atom(CssAtomSet::Counters)]
	pub name: T![Function],
	pub params: CountersFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CountersFunctionParams<'a>(
	pub T![Ident],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub T![String],
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<CounterStyle<'a>>,
);

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counter> = <counter()> | <counters()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Counter<'a> {
	Counter(Box<'a, CounterFunction<'a>>),
	Counters(Box<'a, CountersFunction<'a>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Counter, "counter(foo)");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counter(foo,upper-latin)");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counters(foo,'bar')");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counters(foo,'bar',upper-latin)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Counter, "counter('bar')");
		assert_parse_error!(CssAtomSet::ATOMS, Counter, "counters('bar')");
	}
}
