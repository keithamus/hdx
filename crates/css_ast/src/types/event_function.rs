use super::prelude::*;
use crate::KeypressFunction;

/// <https://drafts.csswg.org/animation-triggers-1/#typedef-event-function>
///
/// ```text,ignore
/// <event-function> = <click()> | <dblclick()> | <pointerenter()> | <pointerleave()> | <pointerup()>
///   | <pointerdown()> | <keypress()> | <focusin()> | <focusout()> | <input()> | <change()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum EventFunction {
	Click(#[atom(CssAtomSet::Click)] T![Function], #[semantic_eq(skip)] T![')']),
	Dblclick(#[atom(CssAtomSet::Dblclick)] T![Function], #[semantic_eq(skip)] T![')']),
	Pointerenter(#[atom(CssAtomSet::Pointerenter)] T![Function], #[semantic_eq(skip)] T![')']),
	Pointerleave(#[atom(CssAtomSet::Pointerleave)] T![Function], #[semantic_eq(skip)] T![')']),
	Pointerup(#[atom(CssAtomSet::Pointerup)] T![Function], #[semantic_eq(skip)] T![')']),
	Pointerdown(#[atom(CssAtomSet::Pointerdown)] T![Function], #[semantic_eq(skip)] T![')']),
	KeypressFunction(KeypressFunction),
	Focusin(#[atom(CssAtomSet::Focusin)] T![Function], #[semantic_eq(skip)] T![')']),
	Focusout(#[atom(CssAtomSet::Focusout)] T![Function], #[semantic_eq(skip)] T![')']),
	Input(#[atom(CssAtomSet::Input)] T![Function], #[semantic_eq(skip)] T![')']),
	Change(#[atom(CssAtomSet::Change)] T![Function], #[semantic_eq(skip)] T![')']),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, EventFunction, "click()");
		assert_parse!(CssAtomSet::ATOMS, EventFunction, "pointerdown()");
		assert_parse!(CssAtomSet::ATOMS, EventFunction, "change()");
		assert_parse!(CssAtomSet::ATOMS, EventFunction, "keypress()");
		assert_parse!(CssAtomSet::ATOMS, EventFunction, "keypress(\"a b\")");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, EventFunction, "click");
		assert_parse_error!(CssAtomSet::ATOMS, EventFunction, "click(foo)");
		assert_peek_false!(CssAtomSet::ATOMS, EventFunction, "activate()");
	}
}
