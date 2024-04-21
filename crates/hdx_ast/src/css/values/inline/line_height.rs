use crate::{
	css::units::{CSSFloat, LengthPercentage},
	Parsable, Value, Writable,
};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[value(Inherits)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum LineHeight {
	#[default]
	Normal, // atom!("normal")
	#[parsable(Number, Check::Range(0.0..))]
	Number(CSSFloat),
	#[parsable(Dimension, FromToken, Check::Range(0.0..))]
	LengthPercentage(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(LineHeight, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LineHeight, "0");
		assert_parse!(LineHeight, "0px");
		assert_parse!(LineHeight, "10px");
		assert_parse!(LineHeight, "1.25");
		assert_parse!(LineHeight, "normal");
	}
}
