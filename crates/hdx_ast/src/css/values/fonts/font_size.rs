use crate::{css::units::LengthPercentage, Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[value(Inherits)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum FontSize {
	XxSmall, // atom!("xx-small")
	XSmall,  // atom!("x-small")
	Small,   // atom!("small")
	#[default]
	Medium, // atom!("medium")
	Large,   // atom!("large")
	XLarge,  // atom!("x-large")
	XxLarge, // atom!("xx-large")
	Larger,  // atom!("larger")
	Smaller, // atom!("smaller")
	#[parsable(DimensionOrZero, FromToken, Check::Range(0.0..))]
	LengthPercentage(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontSize, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontSize, "0");
		assert_parse!(FontSize, "10px");
		assert_parse!(FontSize, "xx-large");
	}
}
