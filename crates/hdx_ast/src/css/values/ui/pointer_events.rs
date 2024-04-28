use hdx_derive::{Atomizable, Parsable, Value, Writable};

#[derive(Value, Default, Parsable, Writable, Atomizable, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum PointerEvents {
	#[default]
	Auto, // atom!("auto")
	None, // atom!("none")
}
