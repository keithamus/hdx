use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextDecorationStyle {
	#[default]
	Solid, // atom!("solid"),
	Double, // atom!("double")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Wavy,   // atom!("wavy")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextDecorationStyle, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextDecorationStyle, "solid");
		assert_parse!(TextDecorationStyle, "dotted");
	}
}
