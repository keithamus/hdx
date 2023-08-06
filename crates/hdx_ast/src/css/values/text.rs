#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Expr, Shorthand};
use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-text-4/#propdef-text-align
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextAlignValue {
	#[default]
	Start, // atom!("start")
	End,         // atom!("end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Center,      // atom!("center")
	Justify,     // atom!("justify")
	MatchParent, // atom!("match-parent")
	JustifyAll,  // atom!("justify-all")
	             // TODO: Custom?
}

// https://drafts.csswg.org/css-text-4/#propdef-text-align-all
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextAlignAllValue {
	#[default]
	Start, // atom!("start")
	End,         // atom!("end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Center,      // atom!("center")
	Justify,     // atom!("justify")
	MatchParent, // atom!("match-parent")
}

// https://drafts.csswg.org/css-text-4/#propdef-text-align-last
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextAlignLastValue {
	#[default]
	Auto, // atom!("auto")
	Start,       // atom!("start")
	End,         // atom!("end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Center,      // atom!("center")
	Justify,     // atom!("justify")
	MatchParent, // atom!("match-parent")
}

// https://drafts.csswg.org/css-text-4/#propdef-text-wrap
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextWrapValue {
	#[default]
	Wrap, // atom!("wrap")
	Nowrap,  // atom!("nowrap")
	Balance, // atom!("balance")
	Stable,  // atom!("stable")
	Pretty,  // atom!("pretty")
}

// https://drafts.csswg.org/css-text-4/#propdef-white-space-collapse
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WhiteSpaceCollapseValue {
	#[default]
	Collapse, // atom!("collapse")
	Discard,        // atom!("discard")
	Preserve,       // atom!("preserve")
	PreserveBreaks, // atom!("preserve-breaks")
	PreserveSpaces, // atom!("preserve-spaces")
	BreakSpaces,    // atom!("break-spaces")
}

// https://drafts.csswg.org/css-text-4/#propdef-white-space-trim
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WhiteSpaceTrimValue {
	#[default]
	None,
	Discard {
		before: bool,
		after: bool,
		inner: bool,
	},
}

// https://drafts.csswg.org/css-text-4/#propdef-white-space
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WhiteSpaceShorthand<'a> {
	#[default]
	Normal,
	Pre,
	Nowrap,
	PreWrap,
	PreLine,
	Expanded {
		collapse: Shorthand<'a, Expr<'a, WhiteSpaceCollapseValue>>,
		wrap: Shorthand<'a, Expr<'a, TextWrapValue>>,
		trim: Shorthand<'a, Expr<'a, WhiteSpaceTrimValue>>,
	},
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<TextAlignValue>(), 1);
		assert_eq!(size_of::<TextAlignLastValue>(), 1);
		assert_eq!(size_of::<TextWrapValue>(), 1);
		assert_eq!(size_of::<WhiteSpaceTrimValue>(), 3);
		assert_eq!(size_of::<WhiteSpaceCollapseValue>(), 1);
		assert_eq!(size_of::<WhiteSpaceShorthand>(), 32);
	}
}
