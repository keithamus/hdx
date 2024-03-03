use crate::{
	css::values::{units::CSSFloat, Todo},
	Atomizable, Parsable, Value, Writable,
};

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-align
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxAlign {
	Start,
	Center,
	End,
	Baseline,
	#[default]
	Stretch,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-direction
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxDirection {
	#[default]
	Normal, // atom!("normal")
	Reverse, // atom!("reverse")
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-flex
#[derive(Value, Parsable, Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BoxFlex(#[parsable(FromToken)] pub CSSFloat);

impl Default for BoxFlex {
	fn default() -> Self {
		Self(0.0.into())
	}
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-flex-group
#[derive(Value, Parsable, Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BoxFlexGroup(#[parsable(FromToken)] pub CSSFloat);

impl Default for BoxFlexGroup {
	fn default() -> Self {
		Self(1.0.into())
	}
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-lines
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxLines {
	#[default]
	Single, // atom!("single")
	Multiple, // atom!("multiple")
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-ordinal-group
#[derive(Value, Parsable, Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BoxOrdinalGroup(#[parsable(FromToken)] pub CSSFloat);

impl Default for BoxOrdinalGroup {
	fn default() -> Self {
		Self(1.0.into())
	}
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-orient
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxOrient {
	Horizontal, // atom!("horizontal")
	Vertical,   // atom!("vertical")
	#[default]
	InlineAxis, // atom!("inline-axis")
	BlockAxis,  // atom!("block-axis")
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/box-orient
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BoxPack {
	#[default]
	Start,   // atom!("start")
	Center,  // atom!("center")
	End,     // atom!("end")
	Justify, // atom!("justify")
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BoxDirection, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BoxAlign, "stretch");
		assert_parse!(BoxFlex, "0");
		assert_parse!(BoxFlex, "3.5");
		assert_parse!(BoxDirection, "reverse");
		assert_parse!(BoxFlexGroup, "1");
		assert_parse!(BoxLines, "multiple");
		assert_parse!(BoxOrdinalGroup, "1");
		assert_parse!(BoxOrient, "block-axis");
		assert_parse!(BoxPack, "justify");
	}
}
