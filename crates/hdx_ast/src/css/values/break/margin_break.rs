use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-margin-break
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MarginBreak {
	#[default]
	Auto, // atom!("auto")
	Keep,    // atom!("keep")
	Discard, // atom!("discard")
}
