use hdx_derive::{Parsable, Value, Writable};

// https://drafts.csswg.org/css-writing-modes/#direction
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Direction {
	#[default]
	Ltr, // atom!("ltr")
	Rtl, // atom!("rtl")
}
