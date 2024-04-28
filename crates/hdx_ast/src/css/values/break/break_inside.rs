use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-break-inside
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BreakInside {
	#[default]
	Auto, // atom!("auto")
	Avoid,       // atom!("avoid")
	AvoidPage,   // atom!("avoid-page")
	AvoidColumn, // atom!("avoid-column")
	AvoidRegion, // atom!("avoid-region")
}
