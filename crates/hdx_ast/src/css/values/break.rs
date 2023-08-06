#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-break-4/#propdef-box-decoration-break
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxDecorationBreakValue {
	#[default]
	Slice, // atom!("slice")
	Clone, // atom!("clone")
}

// https://drafts.csswg.org/css-break-4/#propdef-break-after
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BreakValue {
	#[default]
	Auto, // atom!("auto")
	Avoid,       // atom!("avoid")
	Always,      // atom!("always")
	All,         // atom!("all")
	AvoidPage,   // atom!("avoid-page")
	Page,        // atom!("page")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Recto,       // atom!("recto")
	Verso,       // atom!("verso")
	AvoidColumn, // atom!("avoid-column")
	Column,      // atom!("column")
	AvoidRegion, // atom!("avoid-region")
	Region,      // atom!("region")
}

// https://drafts.csswg.org/css-break-4/#propdef-break-inside
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BreakInsideValue {
	#[default]
	Auto, // atom!("auto")
	Avoid,       // atom!("avoid")
	AvoidPage,   // atom!("avoid-page")
	AvoidColumn, // atom!("avoid-column")
	AvoidRegion, // atom!("avoid-region")
}

// https://drafts.csswg.org/css-break-4/#propdef-margin-break
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MarginBreakValue {
	#[default]
	Auto, // atom!("auto")
	Keep,    // atom!("keep")
	Discard, // atom!("discard")
}
