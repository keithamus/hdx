#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-break-inside
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BreakInside {
	#[default]
	Auto, // atom!("auto")
	Avoid,       // atom!("avoid")
	AvoidPage,   // atom!("avoid-page")
	AvoidColumn, // atom!("avoid-column")
	AvoidRegion, // atom!("avoid-region")
}
