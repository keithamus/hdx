use hdx_derive::{Parsable, Value, Writable};

// https://drafts.csswg.org/css-writing-modes/#direction
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ObjectFit {
	#[default]
	Fill, // atom!("fill")
	Contain,   // atom!("contain")
	Cover,     // atom!("cover")
	None,      // atom!("none")
	ScaleDown, // atom!("scale-down")
}
