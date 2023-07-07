#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
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
}

// https://drafts.csswg.org/css-text/#text-align-all-property
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
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

// https://drafts.csswg.org/css-text/#text-align-all-property
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
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
