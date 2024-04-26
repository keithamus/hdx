use crate::{css::units::CSSFloat, Parsable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum FontWeight {
	#[parsable(Number, Check::Range(1.0..=1000.0), Check::Int)]
	Number(CSSFloat),
	#[default]
	Normal, // atom!("normal")
	Bold,    // atom!("bold")
	Bolder,  // atom!("bolder")
	Lighter, // atom!("lighter")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontWeight, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontWeight, "normal");
		assert_parse!(FontWeight, "1");
		assert_parse!(FontWeight, "1000");
	}
}
